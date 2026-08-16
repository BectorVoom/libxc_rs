//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 314/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk314(t1263: f64, t1272: f64, t1268: f64, t1275: f64, t390: f64, t391: f64, t387: f64, t1397: f64, t392: f64, t306: f64, t130: f64, t242: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1409 = 0.9421211958699838_f64 * t1263;
    let t1411 = 0.3140403986233279_f64 * t1272;
    let t1413 = t1409 - 0.9421211958699838_f64 * t1268 + t1411 + 0.9421211958699838_f64 * t1275;
    let t1416 = 1.0_f64 / t391 / t390;
    let t1417 = t387 * t1416;
    let t1420 = t1413 * t392 - t1417 * t1397 / 2.0_f64;
    let t1421 = t387 * t387;
    let t1422 = 1.0_f64 / t390;
    let t1424 = -t1421 * t1422 + 1.0_f64;
    let t1425 = 1.0_f64 / t1424;
    let t1426 = t1420 * t1425;
    let t1427 = t1426 * t306;
    let t1430 = t242 * t130;
    (t1409, t1411, t1413, t1416, t1417, t1421, t1422, t1424, t1425, t1426, t1427, t1430)
}
