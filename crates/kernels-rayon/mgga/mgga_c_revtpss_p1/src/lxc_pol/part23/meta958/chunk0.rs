//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3213/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3213(t5412: f64, t6628: f64, t1287: f64, t12966: f64, t12987: f64, t17307: f64, t17888: f64, t17955: f64, t20721: f64, t20747: f64, t21430: f64, t21554: f64, t21583: f64, t24713: f64, t24941: f64, t24978: f64, t25002: f64, t3670: f64, t3759: f64, t3767: f64, t3769: f64, t45666: f64, t45715: f64, t5326: f64, t5486: f64, t59492: f64, t6727: f64, t72686: f64, t82881: f64) -> (f64, f64) {
    let t84415 = t5412 * t6628;
    let t84425 = 0.11853808529283920877e2_f64 * t72686 * t21583 - 0.39512695097613069591e1_f64 * t45715 * t25002 - 0.39512695097613069591e1_f64 * t45666 * t82881 * t1287 + 0.39512695097613069591e1_f64 * t17955 * t24978 + 0.39512695097613069591e1_f64 * t17888 * t24978 + 0.39512695097613069591e1_f64 * t59492 * t6727 - 0.11853808529283920877e2_f64 * t12987 * t5486 * t20747 + 0.39512695097613069591e1_f64 * t12966 * t24941 + 0.39512695097613069592e1_f64 * t17307 * t21430 - 0.19756347548806534796e1_f64 * t5326 * t21554 + 0.39512695097613069591e1_f64 * t3767 * t84415 * t3769 + 0.39512695097613069591e1_f64 * t3670 * t3759 * t24713 + 0.79025390195226139182e1_f64 * t3670 * t5486 * t20721;
    (t84415, t84425)
}
