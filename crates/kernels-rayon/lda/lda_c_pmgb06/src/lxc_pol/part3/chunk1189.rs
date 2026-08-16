//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1189/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1189(t118: f64, t5567: f64, t11676: f64, t1366: f64, t5652: f64, t5655: f64, t2349: f64, t3309: f64, t3333: f64, t5649: f64, t11751: f64, t11756: f64, t11758: f64, t11759: f64, t11760: f64, t11761: f64, t11763: f64) -> (f64, f64, f64) {
    let t14306 = t5567 * t118;
    let t14308 = t11676 * t118;
    let t14310 = t5652 * t1366;
    let t14311 = 0.21642082724729686_f64 * t14310;
    let t14312 = t5655 * t1366;
    let t14314 = t2349 * t3309;
    let t14316 = t5649 * t3333;
    let t14318 = t14311 + 0.21642082724729686_f64 * t14312 - 0.09618703433213194_f64 * t14314 - 0.011181742741110338_f64 * t14316 + t11751 - t11756 + t11758 + t11759 + t11760 + t11761 - t11763;
    (t14306, t14308, t14318)
}
