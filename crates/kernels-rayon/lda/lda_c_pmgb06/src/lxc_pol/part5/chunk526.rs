//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 526/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk526(t2675: f64, t2680: f64, t2683: f64, t2685: f64, t117: f64, t118: f64, t123: f64, t125: f64, t1328: f64, t1333: f64, t1337: f64, t1349: f64, t1352: f64, t1356: f64, t1360: f64, t2323: f64, t2327: f64, t2331: f64, t2338: f64, t2454: f64) -> (f64, f64) {
    let t2687 = t2675 + t2680 + t2683 + t2685;
    let t2692 = -t1328 + 0.06301081444628223_f64 * t2323 + t1333 + t1337 - 0.031505407223141116_f64 * t2454 * t118 - 0.06301081444628223_f64 * t2327 - 0.003950778065781896_f64 * t2331 - t1349 - t1352 - t1356 - t1360 + 0.017961351015381915_f64 * t2338 - 0.005388405304614574_f64 * t123 * t125 * t2687 * t117;
    (t2687, t2692)
}
