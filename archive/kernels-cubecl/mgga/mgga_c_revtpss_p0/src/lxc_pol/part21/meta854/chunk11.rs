//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3234/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3234<F: Float>(t1774: F, t487: F, t12646: F, t12713: F, t12732: F, t1285: F, t1287: F, t12975: F, t13143: F, t13149: F, t16751: F, t16756: F, t17837: F, t17840: F, t17955: F, t3552: F, t3588: F, t3666: F, t45634: F, t45654: F, t45659: F, t45697: F, t45718: F, t5332: F, t5412: F, t5449: F, t5459: F, t5463: F, t5464: F, t5494: F, t57264: F, t59096: F) -> F {
    let t60037 = t487 * t1774;
    let t60058 = -F::cast_from(0.39512695097613069591e1_f64) * t45654 * t59096 * t13149 + F::cast_from(0.39512695097613069591e1_f64) * t45659 * t59096 * t13143 + F::cast_from(0.19756347548806534796e1_f64) * t45718 * t17837 + F::cast_from(0.19756347548806534796e1_f64) * t45634 * t17837 + F::cast_from(0.39512695097613069591e1_f64) * t17955 * t17840 + F::cast_from(0.19756347548806534796e1_f64) * t3552 * t5494 - F::cast_from(0.11853808529283920877e2_f64) * t57264 * t60037 * t12646 + F::cast_from(0.13170898365871023197e1_f64) * t5463 * t5332 * t5464 * t12732 + F::cast_from(0.19756347548806534796e1_f64) * t1285 * t5412 * t3588 * t1287 - F::cast_from(0.19756347548806534796e1_f64) * t45697 * t5459 + F::cast_from(0.39512695097613069591e1_f64) * t5463 * t16756 * t12713 - F::cast_from(0.19756347548806534796e1_f64) * t3666 * t16751 - F::cast_from(0.19756347548806534796e1_f64) * t12975 * t5449;
    t60058
}
