//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3212/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3212(t24739: f64, t3153: f64, t1234: f64, t1248: f64, t12717: f64, t12744: f64, t1287: f64, t16756: f64, t17307: f64, t1774: f64, t1822: f64, t20703: f64, t21443: f64, t21513: f64, t21518: f64, t21524: f64, t24713: f64, t24977: f64, t24994: f64, t45859: f64, t45863: f64, t5463: f64, t5465: f64, t5480: f64, t57264: f64, t59674: f64, t59788: f64, t59817: f64, t60037: f64, t68674: f64, t72397: f64) -> f64 {
    let t84362 = t24739 * t3153;
    let t84392 = -0.19756347548806534796e1_f64 * t12744 * t24994 + 0.79025390195226139182e1_f64 * t45859 * t84362 * t5465 - 0.39512695097613069591e1_f64 * t45863 * t84362 * t5480 + 0.39512695097613069591e1_f64 * t12717 * t24713 * t1248 * t1287 + 0.79025390195226139182e1_f64 * t59817 * t21443 - 0.79025390195226139182e1_f64 * t59788 * t21513 + 0.39512695097613069591e1_f64 * t59674 * t21518 - 0.11853808529283920877e2_f64 * t57264 * t60037 * t20703 + 0.19756347548806534796e1_f64 * t68674 * t1822 + 0.79025390195226139182e1_f64 * t17307 * t21524 + 0.39512695097613069591e1_f64 * t5463 * t16756 * t24977 - 0.19756347548806534796e1_f64 * t1234 * t72397 * t1774;
    t84392
}
