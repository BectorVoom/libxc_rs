//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 632/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk632(t1284: f64, t5253: f64, t5012: f64, t1468: f64, t364: f64, t1319: f64, t4998: f64, t363: f64, t309: f64, t5009: f64, t372: f64, t4993: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5254 = t5253 * t1284;
    let t5256 = 37.27051603526593_f64 * t5254 * t5012;
    let t5257 = t364 * t1468;
    let t5258 = t5257 * t1284;
    let t5260 = 9.87466743489671_f64 * t5258 * t5012;
    let t5262 = 3.2915558116322368_f64 * t1319 * t4998;
    let t5266 = t363 * t363;
    let t5267 = 1.0_f64 / t5266;
    let t5272 = t5009 * t309;
    let t5273 = t372 * t4993;
    (t5256, t5260, t5262, t5267, t5272, t5273)
}
