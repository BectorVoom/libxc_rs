//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3034/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3034<F: Float>(t342: F, t378: F, t43400: F, t11173: F, t11247: F, t12094: F, t12127: F, t12132: F, t12133: F, t12146: F, t15780: F, t16393: F, t16409: F, t16505: F, t16506: F, t16520: F, t16523: F, t16574: F, t16581: F, t19526: F, t19569: F, t3287: F, t3318: F, t357: F, t43350: F, t4781: F, t4975: F, t4981: F, t4984: F, t4999: F, t53670: F, t53792: F, t989: F) -> F {
    let t55805 = t342 * t43400 * t378;
    let t55822 = F::cast_from(0.39512695097613069591e1_f64) * t19526 * t12133 - F::cast_from(0.19756347548806534796e1_f64) * t19569 * t12094 + F::cast_from(0.39512695097613069591e1_f64) * t16520 * t16581 - F::cast_from(0.39512695097613069591e1_f64) * t989 * t16505 * t4999 + F::cast_from(0.19756347548806534796e1_f64) * t12127 * t53792 * t3318 - F::cast_from(0.19756347548806534796e1_f64) * t12146 * t16393 + F::cast_from(0.39512695097613069591e1_f64) * t4981 * t15780 * t12132 - F::cast_from(0.65854491829355115987e0_f64) * t55805 * t53670 * t43350 * t11247 * t357 - F::cast_from(0.19756347548806534796e1_f64) * t16523 * t16574 - F::cast_from(0.19756347548806534796e1_f64) * t16506 * t16574 - F::cast_from(0.65854491829355115987e0_f64) * t3287 * t4781 * t4975 * t11173 + F::cast_from(0.79025390195226139182e1_f64) * t989 * t16409 * t4984;
    t55822
}
