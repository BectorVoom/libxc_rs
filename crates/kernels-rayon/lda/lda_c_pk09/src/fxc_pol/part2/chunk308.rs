//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 308/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk308(t1287: f64, t1369: f64, t1243: f64, t1255: f64, t1263: f64, t1272: f64, t1251: f64, t1259: f64, t1268: f64, t1275: f64, t401: f64, t306: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1371 = 18.635258017632964_f64 * t1369 * t1287;
    let t1372 = 6.25_f64 * t1243;
    let t1374 = 2.0833333333333335_f64 * t1255;
    let t1376 = 1.2466946262544771_f64 * t1263;
    let t1378 = 0.41556487541815906_f64 * t1272;
    let t1380 = t1372 - 6.25_f64 * t1251 + t1374 + 6.25_f64 * t1259 + t1376 - 1.2466946262544771_f64 * t1268 + t1378 + 1.2466946262544771_f64 * t1275;
    let t1381 = 1.0_f64 / t401;
    let t1382 = t1380 * t1381;
    let t1383 = t1382 * t306;
    (t1371, t1372, t1374, t1376, t1378, t1380, t1381, t1382, t1383)
}
