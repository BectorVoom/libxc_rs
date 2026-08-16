//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3031/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3031(t15654: f64, t3286: f64, t16543: f64, t3046: f64, t1071: f64, t1087: f64, t1089: f64, t12133: f64, t12146: f64, t12154: f64, t12160: f64, t15780: f64, t16183: f64, t16393: f64, t16410: f64, t16468: f64, t16488: f64, t16537: f64, t16573: f64, t16581: f64, t19603: f64, t3043: f64, t3287: f64, t3288: f64, t43360: f64, t4980: f64, t4984: f64, t4995: f64, t4996: f64, t4999: f64, t55345: f64) -> f64 {
    let t55685 = t15654 * t3286;
    let t55701 = t3046 * t16543;
    let t55711 = -0.39512695097613069591e1_f64 * t12146 * t16468 + 0.19756347548806534796e1_f64 * t1087 * t1071 * t16183 * t1089 + 0.39512695097613069591e1_f64 * t16410 * t16581 - 0.39512695097613069591e1_f64 * t55685 * t3288 - 0.19756347548806534796e1_f64 * t12160 * t16488 - 0.19756347548806534796e1_f64 * t4996 * t15780 * t16573 + 0.39512695097613069591e1_f64 * t3043 * t4980 * t4984 - 0.19756347548806534796e1_f64 * t3043 * t4995 * t4999 - 0.39512695097613069591e1_f64 * t43360 * t16537 - 0.39512695097613069591e1_f64 * t55701 * t3288 - 0.19756347548806534796e1_f64 * t12154 * t16393 - 0.19756347548806534796e1_f64 * t3287 * t55345 * t1089 + 0.39512695097613069591e1_f64 * t19603 * t12133;
    t55711
}
