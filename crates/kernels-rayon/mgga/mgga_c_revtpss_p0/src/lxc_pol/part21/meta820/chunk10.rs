//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3037/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3037(t12077: f64, t1647: f64, t1024: f64, t11202: f64, t11782: f64, t11788: f64, t12080: f64, t12089: f64, t12097: f64, t12154: f64, t15655: f64, t16396: f64, t16399: f64, t16449: f64, t16465: f64, t16468: f64, t16584: f64, t19579: f64, t3075: f64, t3223: f64, t3278: f64, t3283: f64, t3292: f64, t43446: f64, t43456: f64, t4781: f64, t4970: f64, t4975: f64, t5009: f64, t54695: f64, t55612: f64) -> f64 {
    let t55899 = t1647 * t12077;
    let t55926 = -0.39512695097613069591e1_f64 * t43446 * t4781 * t4975 * t11202 - 0.39512695097613069591e1_f64 * t55899 * t12080 - 0.19756347548806534796e1_f64 * t16584 * t12089 - 0.39512695097613069591e1_f64 * t12154 * t16468 + 0.79025390195226139182e1_f64 * t11788 * t16399 - 0.39512695097613069591e1_f64 * t43456 * t55612 * t19579 - 0.39512695097613069591e1_f64 * t15655 * t3292 + 0.19756347548806534796e1_f64 * t3278 * t16465 - 0.19756347548806534796e1_f64 * t3223 * t16396 - 0.19756347548806534796e1_f64 * t1024 * t16449 * t3075 + 0.39512695097613069591e1_f64 * t54695 * t3283 + 0.19756347548806534796e1_f64 * t12097 * t5009 - 0.19756347548806534796e1_f64 * t11782 * t4970;
    t55926
}
