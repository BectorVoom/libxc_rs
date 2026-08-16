//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3025/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3025(t1082: f64, t1089: f64, t12122: f64, t12149: f64, t15670: f64, t19446: f64, t19515: f64, t19520: f64, t19593: f64, t20139: f64, t23837: f64, t3287: f64, t3298: f64, t342: f64, t43154: f64, t43446: f64, t4976: f64, t4977: f64, t4983: f64, t4984: f64, t55747: f64, t55934: f64, t6343: f64, t6365: f64, t67652: f64, t67668: f64, t73: f64, t78554: f64, t79116: f64, t79505: f64, t80264: f64) -> f64 {
    let t80557 = -0.39512695097613069591e1_f64 * t55934 * t6365 + 0.39512695097613069591e1_f64 * t55747 * t20139 - 0.19756347548806534796e1_f64 * t67652 * t4977 + 0.39512695097613069592e1_f64 * t342 * t3298 * t6343 * t4984 - 0.79025390195226139182e1_f64 * t12122 * t19593 * t19520 - 0.11853808529283920877e2_f64 * t43446 * t23837 * t73 * t4976 + 0.79025390195226139182e1_f64 * t12149 * t19446 * t67668 - 0.39512695097613069592e1_f64 * t12122 * t80264 * t4983 - 0.19756347548806534796e1_f64 * t3287 * t79505 * t1089 + 0.79025390195226139182e1_f64 * t15670 * t19515 + 0.15805078039045227836e2_f64 * t43154 * t1082 * t78554 + 0.39512695097613069591e1_f64 * t12149 * t79116 * t1089;
    t80557
}
