//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1024/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1024(t8363: f64, t918: f64, t2364: f64, t8359: f64, t1220: f64, t2354: f64, t2358: f64, t2373: f64, t2380: f64, t2390: f64, t2398: f64, t3185: f64, t3206: f64, t3214: f64, t385: f64, t6401: f64, t6430: f64, t8325: f64, t8331: f64, t8333: f64, t8340: f64, t8342: f64, t8346: f64, t8351: f64, t8355: f64, t8360: f64) -> (f64, f64) {
    let t8364 = t918 * t8363;
    let t8368 = t2364 * t8359;
    let t8371 = 0.85748036236139473944e-3_f64 * t6401 + t8325 + t1220 * t2358 / 36.0_f64 - t8331 - t385 * t8333 / 96.0_f64 - t1220 * t2354 / 18.0_f64 + t8340 / 432.0_f64 + t8342 / 162.0_f64 - 0.42874018118069736972e-3_f64 * t2380 * t8346 - 0.85748036236139473944e-3_f64 * t3185 * t8351 + 0.42874018118069736972e-3_f64 * t3206 * t8355 + 0.11433071498151929859e-2_f64 * t8360 * t2398 + t6430 - 0.47637797908966374413e-4_f64 * t8364 - 0.11433071498151929859e-2_f64 * t3214 * t2390 - 0.22866142996303859718e-2_f64 * t8368 * t2373;
    (t8368, t8371)
}
