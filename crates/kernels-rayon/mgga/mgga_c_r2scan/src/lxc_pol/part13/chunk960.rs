//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 960/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk960(t10958: f64, t3446: f64, t3447: f64, t2312: f64, t3438: f64, t1615: f64, t6855: f64, t166: f64, t269: f64, t1103: f64, t1053: f64, t2317: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10960 = t3446 * t3447 * t10958;
    let t10962 = t3438 * t2312;
    let t10964 = t3446 * t3447 * t10962;
    let t10965 = 0.15243824895787514157e-3_f64 * t10964;
    let t10966 = t6855 * t1615;
    let t10967 = t166 * t269;
    let t10968 = t1103 * t10967;
    let t10969 = t10966 * t10968;
    let t10970 = 0.34200192530023447503e-6_f64 * t10969;
    let t10971 = t1053 * t2317;
    (t10960, t10962, t10965, t10966, t10968, t10970, t10971)
}
