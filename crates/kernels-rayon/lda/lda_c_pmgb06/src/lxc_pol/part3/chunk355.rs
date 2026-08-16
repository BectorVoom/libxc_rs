//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 355/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk355(t410: f64, t56: f64, t69: f64, t1260: f64, t1241: f64, t1247: f64, t1249: f64, t1252: f64, t1255: f64, t1264: f64, t1268: f64) -> (f64, f64, f64) {
    let t1302 = 0.3831677777777778_f64 * t69 * t410 * t56;
    let t1303 = t69 * t1260;
    let t1309 = -t1241 + t1247 + t1249 + t1252 - t1255 + t1302 + 1.1495033333333333_f64 * t1303 + 5.172765_f64 * t69 * t1264 - 1.724255_f64 * t69 * t1268;
    (t1302, t1303, t1309)
}
