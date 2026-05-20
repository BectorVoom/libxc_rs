//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 763/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk763<F: Float>(t1583: F, t30: F, t1468: F, t1940: F, t1963: F, t2403: F, t7091: F, t7750: F, t7783: F, t1544: F, t207: F, t7782: F) -> (F, F, F, F) {
    let t7787 = t30 * t1583;
    let t7794 = F::new(3.0) / F::new(2.0) * t2403 * t7750 + t1940 * t7783 * t30 / F::new(2.0) - t1940 * t7091 * t7787 / F::new(2.0) + t1940 * t1963 * t1468 / F::new(2.0);
    let t7847 = t1963 * t1544;
    let t7850 = t207 * t7782;
    (t7787, t7794, t7847, t7850)
}
