//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3221/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3221(t1269: f64, t13141: f64, t460: f64, t12709: f64, t12723: f64, t12727: f64, t12966: f64, t16771: f64, t16772: f64, t16775: f64, t17192: f64, t17840: f64, t17846: f64, t17848: f64, t17856: f64, t17884: f64, t17888: f64, t17902: f64, t3670: f64, t3759: f64, t45868: f64, t490: f64, t5446: f64, t56479: f64, t57536: f64) -> f64 {
    let t59591 = t460 * t13141 * t1269;
    let t59611 = 0.11853808529283920877e2_f64 * t17846 * t57536 * t17848 + 0.79025390195226139182e1_f64 * t3670 * t3759 * t16771 - 0.19756347548806534796e1_f64 * t45868 * t5446 + 0.39512695097613069591e1_f64 * t17888 * t17840 - 0.11853808529283920877e2_f64 * t59591 * t17856 - 0.19756347548806534796e1_f64 * t12709 * t17884 - 0.19756347548806534796e1_f64 * t12723 * t17884 - 0.39512695097613069591e1_f64 * t12709 * t17902 - 0.39512695097613069591e1_f64 * t12723 * t17902 - 0.19756347548806534796e1_f64 * t17192 * t12727 + 0.65854491829355115987e0_f64 * t56479 * t490 + 0.39512695097613069591e1_f64 * t3670 * t3759 * t16775 + 0.79025390195226139182e1_f64 * t12966 * t16772;
    t59611
}
