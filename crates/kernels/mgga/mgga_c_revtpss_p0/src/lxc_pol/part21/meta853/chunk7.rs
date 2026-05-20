//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3219/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3219<F: Float>(t17482: F, t3153: F, t1284: F, t17331: F, t12646: F, t12699: F, t12702: F, t12734: F, t12737: F, t1288: F, t12966: F, t13133: F, t13156: F, t17307: F, t17811: F, t17815: F, t21483: F, t3670: F, t3751: F, t45859: F, t5230: F, t5436: F, t5463: F, t5465: F, t5470: F, t5478: F, t5480: F, t5486: F, t57373: F, t59241: F) -> (F, F) {
    let t59514 = t17482 * t3153;
    let t59537 = t17331 * t1284;
    let t59544 = F::cast_from(0.39512695097613069591e1_f64) * t5463 * t57373 * t5465 + F::cast_from(0.79025390195226139182e1_f64) * t45859 * t59514 * t21483 - F::cast_from(0.19756347548806534796e1_f64) * t5478 * t57373 * t5480 + F::cast_from(0.39512695097613069591e1_f64) * t3670 * t5486 * t12646 + F::cast_from(0.79025390195226139182e1_f64) * t12966 * t17811 + F::cast_from(0.65854491829355115987e0_f64) * t5436 * t12734 + F::cast_from(0.39512695097613069591e1_f64) * t17307 * t13156 + F::cast_from(0.39512695097613069591e1_f64) * t3670 * t13133 * t5230 + F::cast_from(0.39512695097613069591e1_f64) * t59241 * t3751 + F::cast_from(0.39512695097613069591e1_f64) * t17307 * t12737 + F::cast_from(0.19756347548806534796e1_f64) * t59537 * t1288 + F::cast_from(0.39512695097613069591e1_f64) * t12702 * t17815 + F::cast_from(0.19756347548806534796e1_f64) * t12699 * t5470;
    (t59514, t59544)
}
