//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 357/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk357<F: Float>(t1234: F, t1282: F, t1227: F, t1241: F, t1247: F, t1249: F, t1252: F, t1255: F, t1259: F, t1261: F, t1264: F, t1268: F, t1274: F, t1277: F, t1280: F, t360: F, t370: F, t63: F) -> F {
    let t1283 = t1282 * t1234;
    let t1289 = -t1241 + t1247 + t1249 + t1252 - t1255 + t1259 + t1261 / F::cast_from(3.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t360 * t1264 - t360 * t1268 / F::cast_from(2.0_f64) + t1274 + F::cast_from(1.46904_f64) * t1277 + t1280 + F::cast_from(5.87616_f64) * t63 * t1283 - F::cast_from(1.46904_f64) * t63 * t370 * t1227;
    t1289
}
