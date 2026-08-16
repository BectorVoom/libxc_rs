//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1097/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1097(t26223: f64, t26364: f64, t26485: f64, t26500: f64, t533: f64, t1390: f64, t1983: f64, t1393: f64, t1442: f64, t1459: f64, t1774: f64, t1849: f64, t1869: f64, t22461: f64, t26103: f64, t26157: f64, t26166: f64, t26170: f64, t26178: f64, t26181: f64, t26183: f64, t4037: f64, t5107: f64, t6515: f64, t6517: f64, t6862: f64, t6872: f64, t7681: f64) -> (f64, f64, f64, f64) {
    let t26502 = t26223 + t26364 + t26485 + t26500;
    let t26503 = t533 * t26502;
    let t26504 = t26503 * t1390;
    let t26505 = t1983 * t26504;
    let t26507 = t1393 * t7681 - t1442 * t6862 - 2.0_f64 * t1459 * t22461 - 2.0_f64 * t1459 * t26103 - t1774 * t6515 + t1849 * t6872 - t1869 * t5107 - 2.0_f64 * t4037 * t6517 + t26157 + t26166 + t26170 - t26178 - t26181 - t26183 + t26505;
    (t26502, t26504, t26505, t26507)
}
