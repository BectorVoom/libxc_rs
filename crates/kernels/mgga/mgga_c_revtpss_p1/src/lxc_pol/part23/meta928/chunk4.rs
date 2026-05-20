//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3025/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3025<F: Float>(t1082: F, t1089: F, t12122: F, t12149: F, t15670: F, t19446: F, t19515: F, t19520: F, t19593: F, t20139: F, t23837: F, t3287: F, t3298: F, t342: F, t43154: F, t43446: F, t4976: F, t4977: F, t4983: F, t4984: F, t55747: F, t55934: F, t6343: F, t6365: F, t67652: F, t67668: F, t73: F, t78554: F, t79116: F, t79505: F, t80264: F) -> F {
    let t80557 = -F::cast_from(0.39512695097613069591e1_f64) * t55934 * t6365 + F::cast_from(0.39512695097613069591e1_f64) * t55747 * t20139 - F::cast_from(0.19756347548806534796e1_f64) * t67652 * t4977 + F::cast_from(0.39512695097613069592e1_f64) * t342 * t3298 * t6343 * t4984 - F::cast_from(0.79025390195226139182e1_f64) * t12122 * t19593 * t19520 - F::cast_from(0.11853808529283920877e2_f64) * t43446 * t23837 * t73 * t4976 + F::cast_from(0.79025390195226139182e1_f64) * t12149 * t19446 * t67668 - F::cast_from(0.39512695097613069592e1_f64) * t12122 * t80264 * t4983 - F::cast_from(0.19756347548806534796e1_f64) * t3287 * t79505 * t1089 + F::cast_from(0.79025390195226139182e1_f64) * t15670 * t19515 + F::cast_from(0.15805078039045227836e2_f64) * t43154 * t1082 * t78554 + F::cast_from(0.39512695097613069591e1_f64) * t12149 * t79116 * t1089;
    t80557
}
