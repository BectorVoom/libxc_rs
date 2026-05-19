//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 314/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk314<F: Float>(t1263: F, t1272: F, t1268: F, t1275: F, t390: F, t391: F, t387: F, t1397: F, t392: F, t306: F, t130: F, t242: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t1409 = F::cast_from(0.9421211958699838_f64) * t1263;
    let t1411 = F::cast_from(0.3140403986233279_f64) * t1272;
    let t1413 = t1409 - F::cast_from(0.9421211958699838_f64) * t1268 + t1411 + F::cast_from(0.9421211958699838_f64) * t1275;
    let t1416 = F::new(1.0) / t391 / t390;
    let t1417 = t387 * t1416;
    let t1420 = t1413 * t392 - t1417 * t1397 / F::new(2.0);
    let t1421 = t387 * t387;
    let t1422 = F::new(1.0) / t390;
    let t1424 = -t1421 * t1422 + F::new(1.0);
    let t1425 = F::new(1.0) / t1424;
    let t1426 = t1420 * t1425;
    let t1427 = t1426 * t306;
    let t1430 = t242 * t130;
    (t1409, t1411, t1413, t1416, t1417, t1421, t1422, t1424, t1425, t1426, t1427, t1430)
}
