//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 709/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk709(t2437: f64, t542: f64, t1313: f64, t519: f64, t1251: f64, t2329: f64, t348: f64, t1326: f64, t1245: f64, t1991: f64, t3682: f64, t3706: f64, t4583: f64, t5806: f64, t5837: f64, t6312: f64, t6313: f64, t6316: f64, t6317: f64, t6318: f64, t6319: f64, t6320: f64, t6321: f64, t6325: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6326 = t2437 * t542;
    let t6327 = t1313 * t6326;
    let t6329 = 4.0_f64 / 45.0_f64 * t519 * t6327;
    let t6330 = t1251 * t2329;
    let t6331 = t6330 * t348;
    let t6332 = t1326 * t6331;
    let t6334 = 8.0_f64 / 45.0_f64 * t519 * t6332;
    let t6335 = t1245 * t2329;
    let t6336 = t6335 * t348;
    let t6337 = t1991 * t6336;
    let t6339 = 4.0_f64 / 27.0_f64 * t519 * t6337;
    let t6340 = -t6312 + t6313 + 4.0_f64 / 135.0_f64 * t5806 + 2.0_f64 / 135.0_f64 * t3682 - t3706 - t5837 - t6316 - t6317 + t6318 - t6319 + t6320 - t6321 + t4583 + t6325 - t6329 - t6334 + t6339;
    (t6326, t6327, t6329, t6330, t6331, t6332, t6334, t6335, t6336, t6337, t6339, t6340)
}
