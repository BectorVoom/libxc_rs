//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1161/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1161(t21280: f64, t2127: f64, t6597: f64, t1960: f64, t2468: f64, t6788: f64, t808: f64, t2505: f64, t5215: f64, t12357: f64, t15015: f64, t21269: f64, t21271: f64, t21274: f64, t21276: f64, t21277: f64, t21278: f64, t21279: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21281 = 8.0_f64 / 15.0_f64 * t21280;
    let t21282 = t6597 * t2127;
    let t21283 = 8.0_f64 / 15.0_f64 * t21282;
    let t21285 = 4.0_f64 / 5.0_f64 * t1960 * t2468;
    let t21287 = 4.0_f64 / 5.0_f64 * t6788 * t808;
    let t21289 = 4.0_f64 / 5.0_f64 * t5215 * t2505;
    let t21290 = -t21269 + t12357 + t21271 + 0.004546314527777778_f64 * t15015 - t21274 + t21276 - t21277 + t21278 + t21279 + t21281 + t21283 + t21285 - t21287 + t21289;
    (t21281, t21283, t21285, t21287, t21289, t21290)
}
