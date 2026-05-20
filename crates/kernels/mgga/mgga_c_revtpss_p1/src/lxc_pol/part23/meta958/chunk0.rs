//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3213/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3213<F: Float>(t5412: F, t6628: F, t1287: F, t12966: F, t12987: F, t17307: F, t17888: F, t17955: F, t20721: F, t20747: F, t21430: F, t21554: F, t21583: F, t24713: F, t24941: F, t24978: F, t25002: F, t3670: F, t3759: F, t3767: F, t3769: F, t45666: F, t45715: F, t5326: F, t5486: F, t59492: F, t6727: F, t72686: F, t82881: F) -> (F, F) {
    let t84415 = t5412 * t6628;
    let t84425 = F::cast_from(0.11853808529283920877e2_f64) * t72686 * t21583 - F::cast_from(0.39512695097613069591e1_f64) * t45715 * t25002 - F::cast_from(0.39512695097613069591e1_f64) * t45666 * t82881 * t1287 + F::cast_from(0.39512695097613069591e1_f64) * t17955 * t24978 + F::cast_from(0.39512695097613069591e1_f64) * t17888 * t24978 + F::cast_from(0.39512695097613069591e1_f64) * t59492 * t6727 - F::cast_from(0.11853808529283920877e2_f64) * t12987 * t5486 * t20747 + F::cast_from(0.39512695097613069591e1_f64) * t12966 * t24941 + F::cast_from(0.39512695097613069592e1_f64) * t17307 * t21430 - F::cast_from(0.19756347548806534796e1_f64) * t5326 * t21554 + F::cast_from(0.39512695097613069591e1_f64) * t3767 * t84415 * t3769 + F::cast_from(0.39512695097613069591e1_f64) * t3670 * t3759 * t24713 + F::cast_from(0.79025390195226139182e1_f64) * t3670 * t5486 * t20721;
    (t84415, t84425)
}
