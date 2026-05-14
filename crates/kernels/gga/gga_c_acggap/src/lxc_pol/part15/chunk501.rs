//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 501/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk501<F: Float>(t3101: F, t452: F, t381: F, t136: F, t2015: F, t357: F, t1074: F, t2035: F, t1059: F, t576: F, t1062: F, t134: F, t154: F, t352: F, t1068: F, t1072: F, t301: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3102 = t452 * t3101;
    let t3104 = 0.65854491829355115987e0 * t381 * t3102;
    let t3106 = t2015 * t136 * t357;
    let t3107 = 20.0 / 27.0 * t3106;
    let t3109 = t2035 * t136 * t1074;
    let t3110 = 2.0 / 3.0 * t3109;
    let t3111 = t576 * t1059;
    let t3112 = t3111 * t1062;
    let t3114 = t134 * t154;
    let t3115 = t352 * t3114;
    let t3124 = t1068 * t1059;
    let t3125 = t1072 * t301;
    (t3104, t3106, t3107, t3109, t3110, t3111, t3112, t3114, t3115, t3124, t3125)
}
