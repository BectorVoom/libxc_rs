//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1207/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1207(t10556: f64, t192: f64, t1020: f64, t1045: f64, t1055: f64, t10676: f64, t135: f64, t144: f64, t1535: f64, t158: f64, t188: f64, t19757: f64, t19759: f64, t19776: f64, t24194: f64, t24964: f64, t2575: f64, t2671: f64, t2679: f64, t2703: f64, t2714: f64, t2718: f64, t28970: f64, t29118: f64, t29478: f64, t29639: f64, t3461: f64, t3467: f64, t3488: f64, t568: f64, t634: f64, t639: f64, t9020: f64, t9034: f64, t9037: f64, t9043: f64, t9112: f64, t9116: f64) -> f64 {
    let t29644 = t192 * t10556;
    let t29654 = t28970 + t19757 + 18.0_f64 * t2718 * t24964 * t1020 + 18.0_f64 * t2718 * t9116 * t2575 + t19759 + t135 * t144 * (0.65854491829355115987e0_f64 * t29478 * t158 * t188 - 0.65854491829355115987e0_f64 * t10676 * t634 - 0.19756347548806534796e1_f64 * t9020 * t1055 + 0.39512695097613069592e1_f64 * t3461 * t2679 - 0.19756347548806534796e1_f64 * t3461 * t2703 + 0.39512695097613069591e1_f64 * t2671 * t3467 - 0.11853808529283920877e2_f64 * t1045 * t9034 + 0.79025390195226139182e1_f64 * t1045 * t9037 - 0.19756347548806534796e1_f64 * t2671 * t3488 + 0.39512695097613069592e1_f64 * t1045 * t9043 + t29639) * t639 + 6.0_f64 * t2718 * t29644 * t568 + 9.0_f64 * t1535 * t9112 * t2575 + t19776 + t29118 + 36.0_f64 * t2718 * t2714 * t24194;
    t29654
}
