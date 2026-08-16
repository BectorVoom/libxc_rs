//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 461/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk461(t1291: f64, t1296: f64, t2236: f64, t2238: f64, t2241: f64, t2255: f64, t378: f64, t384: f64, t74: f64, t787: f64, t387: f64) -> (f64, f64) {
    let t2257 = -t1291 * t787 + 2.0_f64 * t1296 * t2241 + t2236 * t74 - t2238 * t384 - t378 * t2255;
    let t2258 = t2257 * t387;
    (t2257, t2258)
}
