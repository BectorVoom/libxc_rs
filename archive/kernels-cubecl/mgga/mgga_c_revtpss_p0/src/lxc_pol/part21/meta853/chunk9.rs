//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3221/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3221<F: Float>(t1269: F, t13141: F, t460: F, t12709: F, t12723: F, t12727: F, t12966: F, t16771: F, t16772: F, t16775: F, t17192: F, t17840: F, t17846: F, t17848: F, t17856: F, t17884: F, t17888: F, t17902: F, t3670: F, t3759: F, t45868: F, t490: F, t5446: F, t56479: F, t57536: F) -> F {
    let t59591 = t460 * t13141 * t1269;
    let t59611 = F::cast_from(0.11853808529283920877e2_f64) * t17846 * t57536 * t17848 + F::cast_from(0.79025390195226139182e1_f64) * t3670 * t3759 * t16771 - F::cast_from(0.19756347548806534796e1_f64) * t45868 * t5446 + F::cast_from(0.39512695097613069591e1_f64) * t17888 * t17840 - F::cast_from(0.11853808529283920877e2_f64) * t59591 * t17856 - F::cast_from(0.19756347548806534796e1_f64) * t12709 * t17884 - F::cast_from(0.19756347548806534796e1_f64) * t12723 * t17884 - F::cast_from(0.39512695097613069591e1_f64) * t12709 * t17902 - F::cast_from(0.39512695097613069591e1_f64) * t12723 * t17902 - F::cast_from(0.19756347548806534796e1_f64) * t17192 * t12727 + F::cast_from(0.65854491829355115987e0_f64) * t56479 * t490 + F::cast_from(0.39512695097613069591e1_f64) * t3670 * t3759 * t16775 + F::cast_from(0.79025390195226139182e1_f64) * t12966 * t16772;
    t59611
}
