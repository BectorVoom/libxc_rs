//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3175/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3175(t11668: f64, t11692: f64, t1214: f64, t1227: f64, t14706: f64, t15470: f64, t15474: f64, t15560: f64, t15564: f64, t15594: f64, t15615: f64, t15681: f64, t15740: f64, t1735: f64, t248: f64, t3506: f64, t3508: f64, t3516: f64, t3577: f64, t3578: f64, t4582: f64, t4889: f64, t4972: f64, t5030: f64, t50992: f64, t51002: f64, t52609: f64, t52619: f64, t52766: f64, t52879: f64, t55662: f64, t5971: f64, t61855: f64, t61910: f64, t62044: f64, t65264: f64) -> f64 {
    let t65764 = -t15594 * t5030 / 1152.0_f64 + t3506 * t248 * t1214 * t65264 * t3508 / 768.0_f64 - t1227 * t4582 * t4972 * t55662 / 2304.0_f64 - t1227 * t4582 * t15615 * t62044 / 768.0_f64 - t52879 * t15560 / 1152.0_f64 + t52766 * t15564 / 2304.0_f64 - t15740 * t15470 / 1152.0_f64 - t15740 * t15474 / 2304.0_f64 - t1227 * t4582 * t15615 * t61910 / 768.0_f64 - t1227 * t4582 * t50992 * t61855 / 192.0_f64 - 5.0_f64 / 15552.0_f64 * t52609 - 2.0_f64 / 81.0_f64 * t4889 * t15681 + 5.0_f64 / 384.0_f64 * t1227 * t4582 * t51002 * t61855 - 5.0_f64 / 13824.0_f64 * t11692 * t11668 * t5971 * t3516 - t52619 / 3456.0_f64 - t3577 * t3578 * t1735 * t14706 / 2304.0_f64;
    t65764
}
