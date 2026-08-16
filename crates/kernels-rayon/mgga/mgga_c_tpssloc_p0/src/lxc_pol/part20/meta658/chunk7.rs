//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2447/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2447(t1041: f64, t4584: f64, t49850: f64, t10422: f64, t14032: f64, t3070: f64, t13969: f64, t14166: f64, t1023: f64, t10390: f64, t10483: f64, t13611: f64, t13762: f64, t14012: f64, t14189: f64, t1539: f64, t2960: f64, t2979: f64, t3048: f64, t3071: f64, t42388: f64, t43143: f64, t43155: f64, t43157: f64, t43161: f64, t47726: f64, t973: f64) -> f64 {
    let t50047 = t1041 * t49850 * t4584;
    let t50048 = t50047 / 3456.0_f64;
    let t50056 = t3070 * t10422 * t14032;
    let t50062 = t1041 * t13969 * t14166;
    let t50066 = t973 * t2979 * t47726 / 6.0_f64 - 2.0_f64 / 27.0_f64 * t2960 * t14012 + t42388 * t3071 * t1539 * t10483 / 768.0_f64 - t43143 / 216.0_f64 + t50048 + t10390 * t13762 / 768.0_f64 + t3070 * t3071 * t13611 * t1023 / 1536.0_f64 + t50056 / 2304.0_f64 - 11.0_f64 / 162.0_f64 * t43155 - 5.0_f64 / 162.0_f64 * t43157 - t43161 / 4608.0_f64 + t50062 / 384.0_f64 - 5.0_f64 / 324.0_f64 * t3048 * t14189;
    t50066
}
