//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1712/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1712<F: Float>(t1076: F, t11121: F, t1652: F, t1680: F, t20175: F, t20178: F, t20191: F, t20204: F, t20211: F, t23603: F, t23959: F, t3058: F, t42060: F, t4752: F, t6245: F, t6251: F, t6259: F, t6350: F, t6351: F, t6392: F, t64687: F, t68022: F, t68144: F, t80173: F, t80810: F, t80901: F, t80921: F, t89312: F, t89320: F, t996: F) -> F {
    let t89437 = F::cast_from(0.15805078039045227836e2_f64) * t42060 * t996 * t89320 - F::cast_from(0.79025390195226139183e1_f64) * t80173 * t1652 - F::cast_from(0.26341796731742046395e1_f64) * t80921 * t1652 + F::cast_from(0.15805078039045227836e2_f64) * t20175 * t6351 - F::cast_from(0.23707617058567841754e2_f64) * t1076 * t11121 * t6350 * t6392 + F::cast_from(0.79025390195226139183e1_f64) * t20204 * t6251 + F::cast_from(0.15805078039045227836e2_f64) * t20191 * t6251 + F::cast_from(0.15805078039045227836e2_f64) * t4752 * t23603 - F::cast_from(0.26341796731742046395e1_f64) * t80810 * t1652 + F::cast_from(0.15805078039045227836e2_f64) * t68144 * t6245 + F::cast_from(0.39512695097613069591e1_f64) * t3058 * t996 * t89312 - F::cast_from(0.39512695097613069592e1_f64) * t20211 * t6259 + F::cast_from(0.79025390195226139183e1_f64) * t20178 * t6351 + F::cast_from(0.79025390195226139183e1_f64) * t64687 * t6245 + F::cast_from(0.26341796731742046395e1_f64) * t23959 * t1680 - F::cast_from(0.79025390195226139183e1_f64) * t80901 * t1652 + F::cast_from(0.79025390195226139183e1_f64) * t68022 * t6245;
    t89437
}
