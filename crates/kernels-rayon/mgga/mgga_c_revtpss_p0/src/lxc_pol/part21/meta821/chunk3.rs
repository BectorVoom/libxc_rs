//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3041/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3041(t12166: f64, t1647: f64, t1043: f64, t1087: f64, t1089: f64, t11620: f64, t11782: f64, t12032: f64, t12074: f64, t12137: f64, t12169: f64, t15717: f64, t16450: f64, t16552: f64, t16554: f64, t16559: f64, t16561: f64, t1668: f64, t1685: f64, t3223: f64, t3302: f64, t357: f64, t42278: f64, t43446: f64, t4857: f64, t4893: f64, t4954: f64, t4981: f64, t4983: f64, t4996: f64, t5005: f64, t53904: f64, t54479: f64, t54909: f64, t55499: f64) -> f64 {
    let t56017 = t1647 * t12166;
    let t56041 = -0.19756347548806534796e1_f64 * t11782 * t5005 - 0.11853808529283920877e2_f64 * t43446 * t15717 * t1043 * t1089 + 0.11853808529283920877e2_f64 * t16552 * t55499 * t53904 + 0.11853808529283920877e2_f64 * t16552 * t54479 * t16554 - 0.65854491829355115987e0_f64 * t42278 * t1685 + 0.39512695097613069591e1_f64 * t56017 * t12169 + 0.39512695097613069591e1_f64 * t4981 * t54909 * t4983 + 0.19756347548806534796e1_f64 * t4954 * t12137 - 0.65854491829355115987e0_f64 * t4996 * t4893 * t3302 * t11620 * t357 - 0.39512695097613069591e1_f64 * t3223 * t16450 - 0.19756347548806534796e1_f64 * t4857 * t12074 + 0.65854491829355115987e0_f64 * t1087 * t12032 * t1668 * t1089 - 0.11853808529283920877e2_f64 * t16559 * t54479 * t16561;
    t56041
}
