//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3037/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3037<F: Float>(t12077: F, t1647: F, t1024: F, t11202: F, t11782: F, t11788: F, t12080: F, t12089: F, t12097: F, t12154: F, t15655: F, t16396: F, t16399: F, t16449: F, t16465: F, t16468: F, t16584: F, t19579: F, t3075: F, t3223: F, t3278: F, t3283: F, t3292: F, t43446: F, t43456: F, t4781: F, t4970: F, t4975: F, t5009: F, t54695: F, t55612: F) -> F {
    let t55899 = t1647 * t12077;
    let t55926 = -F::cast_from(0.39512695097613069591e1_f64) * t43446 * t4781 * t4975 * t11202 - F::cast_from(0.39512695097613069591e1_f64) * t55899 * t12080 - F::cast_from(0.19756347548806534796e1_f64) * t16584 * t12089 - F::cast_from(0.39512695097613069591e1_f64) * t12154 * t16468 + F::cast_from(0.79025390195226139182e1_f64) * t11788 * t16399 - F::cast_from(0.39512695097613069591e1_f64) * t43456 * t55612 * t19579 - F::cast_from(0.39512695097613069591e1_f64) * t15655 * t3292 + F::cast_from(0.19756347548806534796e1_f64) * t3278 * t16465 - F::cast_from(0.19756347548806534796e1_f64) * t3223 * t16396 - F::cast_from(0.19756347548806534796e1_f64) * t1024 * t16449 * t3075 + F::cast_from(0.39512695097613069591e1_f64) * t54695 * t3283 + F::cast_from(0.19756347548806534796e1_f64) * t12097 * t5009 - F::cast_from(0.19756347548806534796e1_f64) * t11782 * t4970;
    t55926
}
