//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3217/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3217(t1287: f64, t17192: f64, t17861: f64, t17934: f64, t21427: f64, t21443: f64, t21452: f64, t21484: f64, t21491: f64, t21513: f64, t21518: f64, t21521: f64, t21596: f64, t24919: f64, t3746: f64, t3755: f64, t5459: f64, t5463: f64, t5465: f64, t57465: f64, t59681: f64, t59749: f64, t59788: f64, t60019: f64, t6735: f64, t72267: f64, t82859: f64, t83330: f64) -> f64 {
    let t84570 = -0.39512695097613069591e1_f64 * t59788 * t21484 - 0.39512695097613069591e1_f64 * t17192 * t21491 + 0.79025390195226139182e1_f64 * t21452 * t21596 - 0.19756347548806534796e1_f64 * t3755 * t83330 * t1287 + 0.13170898365871023197e1_f64 * t5463 * t82859 * t5465 - 0.19756347548806534796e1_f64 * t72267 * t5459 + 0.79025390195226139182e1_f64 * t60019 * t21443 - 0.79025390195226139182e1_f64 * t59749 * t21513 + 0.39512695097613069591e1_f64 * t59681 * t21518 + 0.39512695097613069591e1_f64 * t17934 * t21427 + 0.19756347548806534796e1_f64 * t3746 * t24919 + 0.19756347548806534796e1_f64 * t17861 * t6735 - 0.11853808529283920877e2_f64 * t57465 * t21521;
    t84570
}
