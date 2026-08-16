//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2602/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2602(t19051: f64, t4993: f64, t11784: f64, t1227: f64, t21762: f64, t248: f64, t11721: f64, t6218: f64, t11668: f64, t11692: f64, t15503: f64, t15700: f64, t1735: f64, t18241: f64, t19058: f64, t3515: f64, t3577: f64, t3578: f64, t45114: f64, t45197: f64, t4582: f64, t4972: f64, t4980: f64, t52548: f64, t52732: f64, t52897: f64, t5392: f64, t65464: f64, t65819: f64, t65881: f64, t65963: f64, t66533: f64, t70321: f64) -> (f64, f64) {
    let t72556 = t19051 * t4993;
    let t72560 = t1227 * t248 * t11784 * t21762;
    let t72577 = t6218 * t11721;
    let t72593 = t65819 / 3456.0_f64 - t72556 / 2304.0_f64 + 5.0_f64 / 3456.0_f64 * t72560 - t15503 * t19058 / 96.0_f64 - 5.0_f64 / 4608.0_f64 * t11692 * t11668 * t15700 * t52548 * t5392 - t1227 * t4582 * t4972 * t70321 / 768.0_f64 - t3577 * t3578 * t1735 * t18241 / 1536.0_f64 + 3.0_f64 / 512.0_f64 * t45197 * t52897 * t72577 * t15700 - 3.0_f64 / 512.0_f64 * t45114 * t52897 * t65464 * t15700 - t52732 - t3515 * t4582 * t66533 * t1735 / 1024.0_f64 - t65881 / 1536.0_f64 + t65963 * t4980 / 512.0_f64;
    (t72577, t72593)
}
