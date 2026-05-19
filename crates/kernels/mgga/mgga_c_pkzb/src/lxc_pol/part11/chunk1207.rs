//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1207/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1207<F: Float>(t10556: F, t192: F, t1020: F, t1045: F, t1055: F, t10676: F, t135: F, t144: F, t1535: F, t158: F, t188: F, t19757: F, t19759: F, t19776: F, t24194: F, t24964: F, t2575: F, t2671: F, t2679: F, t2703: F, t2714: F, t2718: F, t28970: F, t29118: F, t29478: F, t29639: F, t3461: F, t3467: F, t3488: F, t568: F, t634: F, t639: F, t9020: F, t9034: F, t9037: F, t9043: F, t9112: F, t9116: F) -> F {
    let t29644 = t192 * t10556;
    let t29654 = t28970 + t19757 + F::new(18.0) * t2718 * t24964 * t1020 + F::new(18.0) * t2718 * t9116 * t2575 + t19759 + t135 * t144 * (F::cast_from(0.65854491829355115987e0_f64) * t29478 * t158 * t188 - F::cast_from(0.65854491829355115987e0_f64) * t10676 * t634 - F::cast_from(0.19756347548806534796e1_f64) * t9020 * t1055 + F::cast_from(0.39512695097613069592e1_f64) * t3461 * t2679 - F::cast_from(0.19756347548806534796e1_f64) * t3461 * t2703 + F::cast_from(0.39512695097613069591e1_f64) * t2671 * t3467 - F::cast_from(0.11853808529283920877e2_f64) * t1045 * t9034 + F::cast_from(0.79025390195226139182e1_f64) * t1045 * t9037 - F::cast_from(0.19756347548806534796e1_f64) * t2671 * t3488 + F::cast_from(0.39512695097613069592e1_f64) * t1045 * t9043 + t29639) * t639 + F::new(6.0) * t2718 * t29644 * t568 + F::new(9.0) * t1535 * t9112 * t2575 + t19776 + t29118 + F::new(36.0) * t2718 * t2714 * t24194;
    t29654
}
