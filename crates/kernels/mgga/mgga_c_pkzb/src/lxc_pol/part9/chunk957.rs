//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 957/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk957<F: Float>(t2381: F, t8350: F, t2396: F, t8349: F, t1235: F, t2023: F, t46: F, t2394: F, t1229: F, t5939: F, t918: F, t2364: F, t1220: F, t2354: F, t2358: F, t2373: F, t2380: F, t2390: F, t2398: F, t3185: F, t3206: F, t3214: F, t385: F, t6401: F, t6430: F, t8325: F, t8331: F, t8333: F, t8340: F, t8342: F, t8346: F) -> (F, F, F, F, F) {
    let t8351 = t2381 * t8350;
    let t8354 = t8349 * t2396;
    let t8355 = t2381 * t8354;
    let t8358 = t1235 * t2023;
    let t8359 = t8358 * t46;
    let t8360 = t2394 * t8359;
    let t8363 = t5939 * t1229;
    let t8364 = t918 * t8363;
    let t8368 = t2364 * t8359;
    let t8371 = 0.85748036236139473944e-3 * t6401 + t8325 + t1220 * t2358 / 36.0 - t8331 - t385 * t8333 / 96.0 - t1220 * t2354 / 18.0 + t8340 / 432.0 + t8342 / 162.0 - 0.42874018118069736972e-3 * t2380 * t8346 - 0.85748036236139473944e-3 * t3185 * t8351 + 0.42874018118069736972e-3 * t3206 * t8355 + 0.11433071498151929859e-2 * t8360 * t2398 + t6430 - 0.47637797908966374413e-4 * t8364 - 0.11433071498151929859e-2 * t3214 * t2390 - 0.22866142996303859718e-2 * t8368 * t2373;
    (t8354, t8358, t8360, t8368, t8371)
}
