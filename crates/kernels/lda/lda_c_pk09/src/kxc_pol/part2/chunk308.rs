//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 308/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk308<F: Float>(t1287: F, t1369: F, t1243: F, t1255: F, t1263: F, t1272: F, t1251: F, t1259: F, t1268: F, t1275: F, t401: F, t306: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1371 = F::cast_from(18.635258017632964_f64) * t1369 * t1287;
    let t1372 = F::new(6.25) * t1243;
    let t1374 = F::cast_from(2.0833333333333335_f64) * t1255;
    let t1376 = F::cast_from(1.2466946262544771_f64) * t1263;
    let t1378 = F::cast_from(0.41556487541815906_f64) * t1272;
    let t1380 = t1372 - F::new(6.25) * t1251 + t1374 + F::new(6.25) * t1259 + t1376 - F::cast_from(1.2466946262544771_f64) * t1268 + t1378 + F::cast_from(1.2466946262544771_f64) * t1275;
    let t1381 = F::new(1.0) / t401;
    let t1382 = t1380 * t1381;
    let t1383 = t1382 * t306;
    (t1371, t1372, t1374, t1376, t1378, t1380, t1381, t1382, t1383)
}
