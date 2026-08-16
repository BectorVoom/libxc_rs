//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 353/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk353(t1246: f64, t1268: f64, t1252: f64, t538: f64, t1256: f64, t1240: f64, t1241: f64, t1248: f64, t1254: f64, t1258: f64, t1263: f64, t1264: f64, t25: f64) -> (f64, f64, f64, f64) {
    let t1269 = t1268 * t1246;
    let t1272 = t538 * t1252;
    let t1275 = t538 * t1256;
    let t1278 = t1240 + 0.023994444444444443_f64 * t1241 - 0.023994444444444443_f64 * t1248 + 0.07198333333333333_f64 * t1254 - 0.035991666666666665_f64 * t1258 + t1263 + 0.008888888888888889_f64 * t1264 - 0.0022222222222222222_f64 * t25 * t1269 + 0.013333333333333334_f64 * t25 * t1272 - 0.006666666666666667_f64 * t25 * t1275;
    (t1269, t1272, t1275, t1278)
}
