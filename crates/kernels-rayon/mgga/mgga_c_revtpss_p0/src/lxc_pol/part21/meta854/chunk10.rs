//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3233/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3233(t1204: f64, t17852: f64, t1209: f64, t1284: f64, t5412: f64, t17845: f64, t17306: f64, t3754: f64, t1234: f64, t1248: f64, t12719: f64, t12741: f64, t1287: f64, t13112: f64, t17178: f64, t17345: f64, t17633: f64, t17821: f64, t17849: f64, t17856: f64, t17864: f64, t17883: f64, t17934: f64, t3552: f64, t3584: f64, t3755: f64, t3756: f64, t44421: f64, t45666: f64, t5436: f64, t5443: f64, t5477: f64, t5481: f64, t59187: f64) -> f64 {
    let t59987 = t1204 * t17852;
    let t60008 = t1209 * t1284 * t5412;
    let t60013 = t1204 * t17845;
    let t60019 = t17306 * t3754;
    let t60022 = 0.39512695097613069591e1_f64 * t17934 * t13112 - 0.11853808529283920877e2_f64 * t59987 * t17856 - 0.11853808529283920877e2_f64 * t45666 * t17345 * t1248 * t1287 + 0.19756347548806534796e1_f64 * t5436 * t12741 - 0.39512695097613069591e1_f64 * t17864 * t17178 - 0.19756347548806534796e1_f64 * t3552 * t5477 * t5481 - 0.19756347548806534796e1_f64 * t3755 * t59187 * t1287 - 0.19756347548806534796e1_f64 * t1234 * t17821 * t3584 - 0.39512695097613069591e1_f64 * t60008 * t3756 + 0.39512695097613069591e1_f64 * t44421 * t5443 + 0.11853808529283920877e2_f64 * t60013 * t17849 - 0.19756347548806534796e1_f64 * t3755 * t17633 * t17883 + 0.39512695097613069591e1_f64 * t60019 * t12719;
    t60022
}
