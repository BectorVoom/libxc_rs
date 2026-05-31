//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1079/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1079<F: Float>(t32276: F, t33964: F, t1885: F, t32284: F, t246: F, t5704: F, t32289: F, t8591: F, t1916: F, t8614: F, t1518: F, t32374: F) -> (F, F, F, F, F, F, F) {
    let t33965 = t32276 * t33964;
    let t33967 = t32284 * t1885;
    let t33969 = t246 * t5704;
    let t33970 = t32289 * t33969;
    let t33971 = t8591 * t33970;
    let t34010 = t1916 * t8614;
    let t34011 = F::cast_from(3.0_f64) * t34010;
    let t34012 = t32374 * t1518;
    (t33965, t33967, t33969, t33970, t33971, t34011, t34012)
}
