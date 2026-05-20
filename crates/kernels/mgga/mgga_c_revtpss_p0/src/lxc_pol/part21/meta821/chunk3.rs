//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3041/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3041<F: Float>(t12166: F, t1647: F, t1043: F, t1087: F, t1089: F, t11620: F, t11782: F, t12032: F, t12074: F, t12137: F, t12169: F, t15717: F, t16450: F, t16552: F, t16554: F, t16559: F, t16561: F, t1668: F, t1685: F, t3223: F, t3302: F, t357: F, t42278: F, t43446: F, t4857: F, t4893: F, t4954: F, t4981: F, t4983: F, t4996: F, t5005: F, t53904: F, t54479: F, t54909: F, t55499: F) -> F {
    let t56017 = t1647 * t12166;
    let t56041 = -F::cast_from(0.19756347548806534796e1_f64) * t11782 * t5005 - F::cast_from(0.11853808529283920877e2_f64) * t43446 * t15717 * t1043 * t1089 + F::cast_from(0.11853808529283920877e2_f64) * t16552 * t55499 * t53904 + F::cast_from(0.11853808529283920877e2_f64) * t16552 * t54479 * t16554 - F::cast_from(0.65854491829355115987e0_f64) * t42278 * t1685 + F::cast_from(0.39512695097613069591e1_f64) * t56017 * t12169 + F::cast_from(0.39512695097613069591e1_f64) * t4981 * t54909 * t4983 + F::cast_from(0.19756347548806534796e1_f64) * t4954 * t12137 - F::cast_from(0.65854491829355115987e0_f64) * t4996 * t4893 * t3302 * t11620 * t357 - F::cast_from(0.39512695097613069591e1_f64) * t3223 * t16450 - F::cast_from(0.19756347548806534796e1_f64) * t4857 * t12074 + F::cast_from(0.65854491829355115987e0_f64) * t1087 * t12032 * t1668 * t1089 - F::cast_from(0.11853808529283920877e2_f64) * t16559 * t54479 * t16561;
    t56041
}
