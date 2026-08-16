//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2441/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2441(t49922: f64, t10408: f64, t10428: f64, t10919: f64, t14152: f64, t14508: f64, t1618: f64, t2771: f64, t2960: f64, t3070: f64, t42573: f64, t42658: f64, t43103: f64, t43110: f64, t4600: f64, t4644: f64, t4650: f64, t47746: f64, t49892: f64, t49894: f64, t49897: f64, t49907: f64, t973: f64, t977: f64) -> f64 {
    let t49923 = t49922 / 2304.0_f64;
    let t49924 = -5.0_f64 / 648.0_f64 * t49892 - t49894 / 768.0_f64 - t49897 / 768.0_f64 + t42573 * t4600 / 96.0_f64 + t14508 * t10428 / 512.0_f64 + 5.0_f64 / 4608.0_f64 * t4644 * t10919 + t49907 - t2960 * t14152 / 6.0_f64 - t973 * t977 * t47746 / 12.0_f64 + 5.0_f64 / 4608.0_f64 * t3070 * t10408 * t4650 * t2771 + 7.0_f64 / 1944.0_f64 * t43103 + t43110 / 216.0_f64 - 209.0_f64 / 2592.0_f64 * t42658 * t1618 - t49923;
    t49924
}
