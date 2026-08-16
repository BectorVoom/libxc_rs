//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1020/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1020(t23598: f64, t996: f64, t1695: f64, t3269: f64, t6392: f64, t1651: f64, t6350: f64, t1079: f64, t6258: f64, t1076: f64, t1647: f64, t1652: f64, t16600: f64, t1696: f64, t19351: f64, t20178: f64, t20204: f64, t20211: f64, t23583: f64, t3058: f64, t4778: f64, t4935: f64, t6245: f64, t6251: f64, t6259: f64, t6345: f64, t6351: f64, t995: f64) -> f64 {
    let t23599 = t996 * t23598;
    let t23603 = t3269 * t1695 * t6392;
    let t23607 = t3269 * t1651 * t6350;
    let t23616 = t1651 * t6392;
    let t23617 = t1079 * t23616;
    let t23620 = t6258 * t1695;
    let t23621 = t1079 * t23620;
    let t23628 = -0.19756347548806534796e1_f64 * t20204 * t1652 + 0.39512695097613069591e1_f64 * t16600 * t6245 - 0.19756347548806534796e1_f64 * t4778 * t6259 - 0.39512695097613069591e1_f64 * t3058 * t23583 - 0.19756347548806534796e1_f64 * t20211 * t1652 - 0.65854491829355115987e0_f64 * t995 * t23599 + 0.39512695097613069591e1_f64 * t1076 * t23603 - 0.39512695097613069591e1_f64 * t995 * t23607 - 0.19756347548806534796e1_f64 * t19351 * t1696 - 0.19756347548806534796e1_f64 * t20178 * t1696 + 0.39512695097613069591e1_f64 * t4935 * t6351 + 0.19756347548806534796e1_f64 * t995 * t23617 + 0.19756347548806534796e1_f64 * t995 * t23621 + 0.39512695097613069591e1_f64 * t4778 * t6251 + 0.19756347548806534796e1_f64 * t1647 * t6345;
    t23628
}
