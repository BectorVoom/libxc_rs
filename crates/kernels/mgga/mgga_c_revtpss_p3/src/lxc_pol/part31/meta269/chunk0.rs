//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1203/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1203<F: Float>(t3: F, t7318: F, t1459: F, t2042: F, t116: F, t1936: F, param_d: F) -> (F, F, F, F) {
    let t7319 = t3 * t7318;
    let t7324 = param_d * t7318;
    let t7329 = F::cast_from(3.0_f64) * t1459 * t2042;
    let t7330 = t116 * t1936;
    (t7319, t7324, t7329, t7330)
}
