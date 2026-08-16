//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 500/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk500(t199: f64, t2443: f64, t108: f64, t2325: f64, t2329: f64, t2334: f64, t2337: f64, t659: f64, t661: f64, t92: f64, t93: f64, t1219: f64, t2257: f64, t2261: f64, t2387: f64, t2391: f64, t2395: f64, t2399: f64, t2404: f64, t2409: f64, t2427: f64, t267: f64) -> (f64, f64, f64) {
    let t2445 = 2.0_f64 / 15.0_f64 * t2443 * t199;
    let t2455 = (20.0_f64 / 9.0_f64 * t92 * t2325 + 4.0_f64 / 3.0_f64 * t659 * t2329 + 20.0_f64 / 9.0_f64 * t93 * t2334 + 4.0_f64 / 3.0_f64 * t661 * t2337) * t108;
    let t2460 = t1219 + t2387 - t2391 + t2395 - t2399 + t2404 + t2409 + t2427 + t2445 - t2455 * t267 / 15.0_f64 + 2.0_f64 / 3.0_f64 * t2257 + 0.12155555555555556_f64 * t2261;
    (t2445, t2455, t2460)
}
