//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 254/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk254<F: Float>(t1049: F, t347: F, t136: F, t357: F, t576: F, t137: F, t154: F, t125: F, t134: F, t352: F, t301: F, t355: F, t721: F, t130: F, t39: F, t14: F, t25: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t1050 = t1049 * t347;
    let t1053 = t576 * t136 * t357;
    let t1054 = t1053 / 6.0;
    let t1055 = t154 * t137;
    let t1059 = t134 * t125;
    let t1060 = t352 * t1059;
    let t1061 = t355 * t301;
    let t1062 = t1061 * t721;
    let t1063 = t1060 * t1062;
    let t1068 = t130 * t39;
    let t1072 = 1.0 / t14 / t25 / 4.0;
    (t1050, t1053, t1054, t1055, t1059, t1060, t1061, t1062, t1063, t1068, t1072)
}
