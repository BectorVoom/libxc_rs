//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3034/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3034(t342: f64, t378: f64, t43400: f64, t11173: f64, t11247: f64, t12094: f64, t12127: f64, t12132: f64, t12133: f64, t12146: f64, t15780: f64, t16393: f64, t16409: f64, t16505: f64, t16506: f64, t16520: f64, t16523: f64, t16574: f64, t16581: f64, t19526: f64, t19569: f64, t3287: f64, t3318: f64, t357: f64, t43350: f64, t4781: f64, t4975: f64, t4981: f64, t4984: f64, t4999: f64, t53670: f64, t53792: f64, t989: f64) -> f64 {
    let t55805 = t342 * t43400 * t378;
    let t55822 = 0.39512695097613069591e1_f64 * t19526 * t12133 - 0.19756347548806534796e1_f64 * t19569 * t12094 + 0.39512695097613069591e1_f64 * t16520 * t16581 - 0.39512695097613069591e1_f64 * t989 * t16505 * t4999 + 0.19756347548806534796e1_f64 * t12127 * t53792 * t3318 - 0.19756347548806534796e1_f64 * t12146 * t16393 + 0.39512695097613069591e1_f64 * t4981 * t15780 * t12132 - 0.65854491829355115987e0_f64 * t55805 * t53670 * t43350 * t11247 * t357 - 0.19756347548806534796e1_f64 * t16523 * t16574 - 0.19756347548806534796e1_f64 * t16506 * t16574 - 0.65854491829355115987e0_f64 * t3287 * t4781 * t4975 * t11173 + 0.79025390195226139182e1_f64 * t989 * t16409 * t4984;
    t55822
}
