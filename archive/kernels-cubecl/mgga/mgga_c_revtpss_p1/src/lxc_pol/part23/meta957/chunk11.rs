//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3212/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3212<F: Float>(t24739: F, t3153: F, t1234: F, t1248: F, t12717: F, t12744: F, t1287: F, t16756: F, t17307: F, t1774: F, t1822: F, t20703: F, t21443: F, t21513: F, t21518: F, t21524: F, t24713: F, t24977: F, t24994: F, t45859: F, t45863: F, t5463: F, t5465: F, t5480: F, t57264: F, t59674: F, t59788: F, t59817: F, t60037: F, t68674: F, t72397: F) -> F {
    let t84362 = t24739 * t3153;
    let t84392 = -F::cast_from(0.19756347548806534796e1_f64) * t12744 * t24994 + F::cast_from(0.79025390195226139182e1_f64) * t45859 * t84362 * t5465 - F::cast_from(0.39512695097613069591e1_f64) * t45863 * t84362 * t5480 + F::cast_from(0.39512695097613069591e1_f64) * t12717 * t24713 * t1248 * t1287 + F::cast_from(0.79025390195226139182e1_f64) * t59817 * t21443 - F::cast_from(0.79025390195226139182e1_f64) * t59788 * t21513 + F::cast_from(0.39512695097613069591e1_f64) * t59674 * t21518 - F::cast_from(0.11853808529283920877e2_f64) * t57264 * t60037 * t20703 + F::cast_from(0.19756347548806534796e1_f64) * t68674 * t1822 + F::cast_from(0.79025390195226139182e1_f64) * t17307 * t21524 + F::cast_from(0.39512695097613069591e1_f64) * t5463 * t16756 * t24977 - F::cast_from(0.19756347548806534796e1_f64) * t1234 * t72397 * t1774;
    t84392
}
