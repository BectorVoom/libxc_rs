//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1321/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1321(t3142: f64, t698: f64, t973: f64, t3147: f64, t10981: f64, t2960: f64, t10263: f64, t1041: f64, t1044: f64, t10860: f64, t10957: f64, t10972: f64, t248: f64, t3043: f64, t3048: f64, t3057: f64, t3098: f64, t3114: f64, t3143: f64, t3148: f64, t41709: f64, t42582: f64, t42586: f64, t42595: f64, t42600: f64) -> f64 {
    let t42610 = t973 * t698 * t3142;
    let t42613 = t973 * t698 * t3147;
    let t42619 = t2960 * t10981;
    let t42621 = -t42582 / 36.0_f64 - t42586 / 1152.0_f64 - 5.0_f64 / 243.0_f64 * t3048 * t10972 + t3114 * t10860 / 768.0_f64 + 5.0_f64 / 1944.0_f64 * t42595 + 19.0_f64 / 432.0_f64 * t10957 * t3057 - 19.0_f64 / 288.0_f64 * t42600 * t3043 - t1041 * t248 * t1044 * t41709 / 192.0_f64 - 19.0_f64 / 216.0_f64 * t10957 * t3098 - t42610 / 216.0_f64 - t42613 / 162.0_f64 + 11.0_f64 / 54.0_f64 * t10263 * t3143 + 22.0_f64 / 81.0_f64 * t10263 * t3148 - t42619 / 27.0_f64;
    t42621
}
