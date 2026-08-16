//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 327/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk327(t1271: f64, t64: f64, t955: f64, t27: f64, t365: f64, t370: f64, t1245: f64, t366: f64, t947: f64, t18: f64, t369: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1272 = t1271 * t64;
    let t1274 = 0.16322666666666666_f64 * t1272 * t955;
    let t1276 = t365 * t370 * t27;
    let t1277 = t1276 * t1245;
    let t1280 = 0.3264533333333333_f64 * t366 * t947;
    let t1282 = 1.0_f64 / t369 / t18;
    (t1272, t1274, t1276, t1277, t1280, t1282)
}
