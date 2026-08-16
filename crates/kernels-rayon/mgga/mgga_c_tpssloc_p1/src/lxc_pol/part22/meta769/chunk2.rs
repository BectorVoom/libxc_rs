//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2612/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2612(t11665: f64, t11719: f64, t11721: f64, t1174: f64, t1196: f64, t1215: f64, t1227: f64, t15740: f64, t18300: f64, t18346: f64, t18360: f64, t18965: f64, t19068: f64, t22154: f64, t44725: f64, t44863: f64, t45002: f64, t4582: f64, t4987: f64, t5005: f64, t5011: f64, t52766: f64, t53034: f64, t66241: f64, t66255: f64, t67060: f64, t70458: f64, t72445: f64, t974: f64) -> f64 {
    let t72911 = 5.0_f64 / 4608.0_f64 * t5005 * t19068 + t53034 + 5.0_f64 / 768.0_f64 * t5005 * t18346 + t66241 / 768.0_f64 + 5.0_f64 / 13824.0_f64 * t1227 * t4582 * t4987 * t70458 + 3.0_f64 / 512.0_f64 * t11719 * t4582 * t18300 * t11721 * t5011 + t44863 * t4582 * t72445 * t44725 * t1215 / 128.0_f64 - t1174 * t974 * t1196 * t67060 / 288.0_f64 - t15740 * t18360 / 768.0_f64 - t11665 * t22154 / 1536.0_f64 + t45002 / 10368.0_f64 + t52766 * t18965 / 1536.0_f64 - t66255 / 768.0_f64;
    t72911
}
