//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1464/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1464(t11655: f64, t11731: f64, t11738: f64, t1174: f64, t11825: f64, t1214: f64, t1227: f64, t15615: f64, t15654: f64, t248: f64, t3490: f64, t3494: f64, t3555: f64, t3587: f64, t39097: f64, t39103: f64, t42468: f64, t43764: f64, t44699: f64, t44725: f64, t44803: f64, t44805: f64, t44811: f64, t44817: f64, t44828: f64, t44836: f64, t44847: f64, t44851: f64, t44858: f64, t44863: f64, t44871: f64, t44873: f64, t4582: f64, t475: f64, t974: f64) -> f64 {
    let t44878 = -7.0_f64 / 486.0_f64 * t44803 + 35.0_f64 / 972.0_f64 * t1174 * t974 * t44805 * t39097 + t44811 / 216.0_f64 + 5.0_f64 / 384.0_f64 * t1227 * t4582 * t15654 * t42468 - 7.0_f64 / 54.0_f64 * t1174 * t974 * t44817 * t39097 + 5.0_f64 / 576.0_f64 * t3490 * t11655 + 5.0_f64 / 2304.0_f64 * t11825 * t3587 + 55.0_f64 / 15552.0_f64 * t1227 * t248 * t44828 * t43764 - t44836 * t248 * t1214 * t44699 * t475 / 3072.0_f64 - t1174 * t974 * t3555 * t39103 / 48.0_f64 - t44847 / 162.0_f64 + t44851 / 1152.0_f64 - t1227 * t4582 * t15615 * t42468 / 128.0_f64 - t44858 * t11731 / 128.0_f64 + t44863 * t248 * t1214 * t44699 * t44725 / 128.0_f64 + t44871 / 192.0_f64 + t11738 * t4582 * t44873 * t3494 / 512.0_f64;
    t44878
}
