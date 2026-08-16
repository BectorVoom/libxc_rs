//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 280/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk280(t338: f64, t403: f64, t442: f64, t1056: f64, t1312: f64, t402: f64, t398: f64, t1216: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t400 = 0.0_f64 < t338;
    let t1313 = t403 * t442;
    let t1314 = t1313 * t1056;
    let t1315 = t1312 * t1314;
    let t1318 = t402 * t402;
    let t1319 = 1.0_f64 / t1318;
    let t1320 = t398 * t1319;
    let t1322 = piecewise3(t400, t1216, -t1216);
    (t1313, t1314, t1315, t1318, t1319, t1320, t1322)
}
