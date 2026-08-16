//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3234/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3234(t1774: f64, t487: f64, t12646: f64, t12713: f64, t12732: f64, t1285: f64, t1287: f64, t12975: f64, t13143: f64, t13149: f64, t16751: f64, t16756: f64, t17837: f64, t17840: f64, t17955: f64, t3552: f64, t3588: f64, t3666: f64, t45634: f64, t45654: f64, t45659: f64, t45697: f64, t45718: f64, t5332: f64, t5412: f64, t5449: f64, t5459: f64, t5463: f64, t5464: f64, t5494: f64, t57264: f64, t59096: f64) -> f64 {
    let t60037 = t487 * t1774;
    let t60058 = -0.39512695097613069591e1_f64 * t45654 * t59096 * t13149 + 0.39512695097613069591e1_f64 * t45659 * t59096 * t13143 + 0.19756347548806534796e1_f64 * t45718 * t17837 + 0.19756347548806534796e1_f64 * t45634 * t17837 + 0.39512695097613069591e1_f64 * t17955 * t17840 + 0.19756347548806534796e1_f64 * t3552 * t5494 - 0.11853808529283920877e2_f64 * t57264 * t60037 * t12646 + 0.13170898365871023197e1_f64 * t5463 * t5332 * t5464 * t12732 + 0.19756347548806534796e1_f64 * t1285 * t5412 * t3588 * t1287 - 0.19756347548806534796e1_f64 * t45697 * t5459 + 0.39512695097613069591e1_f64 * t5463 * t16756 * t12713 - 0.19756347548806534796e1_f64 * t3666 * t16751 - 0.19756347548806534796e1_f64 * t12975 * t5449;
    t60058
}
