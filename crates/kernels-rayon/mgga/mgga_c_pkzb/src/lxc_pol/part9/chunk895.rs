//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 895/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk895(t1306: f64, t135: f64, t2457: f64, t2464: f64, t273: f64, t6243: f64, t6245: f64, t6319: f64, t6322: f64, t6329: f64, t6333: f64, t6358: f64, t6359: f64, t6362: f64, t6498: f64, t6500: f64, t6504: f64, t6601: f64, t955: f64, t957: f64) -> f64 {
    let t6605 = -3.0_f64 * t1306 * t2457 * t2464 * t955 + 2.0_f64 * t135 * t273 * t6359 * t6362 + t135 * t273 * t6601 * t957 - t6243 - t6245 - t6319 + t6322 - t6329 + t6333 + t6358 - t6498 + t6500 - t6504;
    t6605
}
