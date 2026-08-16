//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2461/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2461(t1606: f64, t2402: f64, t973: f64, t10454: f64, t4644: f64, t13950: f64, t3117: f64, t14202: f64, t3048: f64, t14206: f64, t3108: f64, t1025: f64, t1041: f64, t10501: f64, t14085: f64, t1622: f64, t3064: f64, t3098: f64, t43374: f64, t43377: f64, t43382: f64, t43406: f64, t43410: f64, t4582: f64, t47775: f64, t48497: f64) -> f64 {
    let t50425 = t973 * t2402 * t1606;
    let t50429 = t4644 * t10454;
    let t50438 = t3117 * t13950;
    let t50442 = t3048 * t14202;
    let t50443 = t50442 / 1296.0_f64;
    let t50445 = t14206 * t3108;
    let t50452 = 5.0_f64 / 3888.0_f64 * t50425 - 209.0_f64 / 3888.0_f64 * t43410 * t1622 + t50429 / 2304.0_f64 - t43374 / 144.0_f64 + t43377 / 216.0_f64 + t43382 / 3456.0_f64 - t1041 * t4582 * t47775 * t48497 / 192.0_f64 + t50438 / 1152.0_f64 + 5.0_f64 / 4608.0_f64 * t14085 * t3064 + t50443 - 5.0_f64 / 3456.0_f64 * t43406 - t50445 * t1025 / 96.0_f64 - t14085 * t3098 / 768.0_f64 - 5.0_f64 / 2304.0_f64 * t4644 * t10501;
    t50452
}
