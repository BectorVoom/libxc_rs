//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3217/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3217<F: Float>(t1287: F, t17192: F, t17861: F, t17934: F, t21427: F, t21443: F, t21452: F, t21484: F, t21491: F, t21513: F, t21518: F, t21521: F, t21596: F, t24919: F, t3746: F, t3755: F, t5459: F, t5463: F, t5465: F, t57465: F, t59681: F, t59749: F, t59788: F, t60019: F, t6735: F, t72267: F, t82859: F, t83330: F) -> F {
    let t84570 = -F::cast_from(0.39512695097613069591e1_f64) * t59788 * t21484 - F::cast_from(0.39512695097613069591e1_f64) * t17192 * t21491 + F::cast_from(0.79025390195226139182e1_f64) * t21452 * t21596 - F::cast_from(0.19756347548806534796e1_f64) * t3755 * t83330 * t1287 + F::cast_from(0.13170898365871023197e1_f64) * t5463 * t82859 * t5465 - F::cast_from(0.19756347548806534796e1_f64) * t72267 * t5459 + F::cast_from(0.79025390195226139182e1_f64) * t60019 * t21443 - F::cast_from(0.79025390195226139182e1_f64) * t59749 * t21513 + F::cast_from(0.39512695097613069591e1_f64) * t59681 * t21518 + F::cast_from(0.39512695097613069591e1_f64) * t17934 * t21427 + F::cast_from(0.19756347548806534796e1_f64) * t3746 * t24919 + F::cast_from(0.19756347548806534796e1_f64) * t17861 * t6735 - F::cast_from(0.11853808529283920877e2_f64) * t57465 * t21521;
    t84570
}
