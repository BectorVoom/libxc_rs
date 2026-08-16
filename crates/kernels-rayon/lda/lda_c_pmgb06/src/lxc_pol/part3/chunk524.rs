//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 524/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk524(t1241: f64, t1249: f64, t1302: f64, t1303: f64, t2185: f64, t2188: f64, t2191: f64, t2212: f64, t2222: f64, t2245: f64, t2247: f64, t2248: f64, t2249: f64, t69: f64) -> f64 {
    let t2255 = -t1241 + t2185 + t1249 + t2188 + t2191 - t2212 + t1302 + 0.5747516666666667_f64 * t1303 + 0.5747516666666667_f64 * t2245 + 5.172765_f64 * t2247 * t2248 * t2249 - 1.724255_f64 * t69 * t2222;
    t2255
}
