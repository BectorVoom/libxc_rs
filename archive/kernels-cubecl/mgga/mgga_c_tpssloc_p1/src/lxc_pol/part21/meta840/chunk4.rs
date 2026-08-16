//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3017/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3017<F: Float>(t3185: F, t61734: F, t1063: F, t11037: F, t14572: F, t14618: F, t14622: F, t14631: F, t14654: F, t17671: F, t17876: F, t18081: F, t18108: F, t18150: F, t3076: F, t3189: F, t3200: F, t3204: F, t384: F, t4615: F, t4649: F, t4669: F, t4684: F, t4691: F, t47853: F, t50508: F, t50509: F, t5903: F, t5936: F, t5941: F, t62604: F) -> F {
    let t63183 = t61734 * t3185;
    let t63198 = F::cast_from(24.0_f64) * t17671 * t4649 * t50508 * t50509 - t14622 * t3200 * t5936 - F::cast_from(4.0_f64) * t18150 * t3200 * t4684 + F::cast_from(2.0_f64) * t1063 * t17876 - F::cast_from(2.0_f64) * t11037 * t18081 - F::cast_from(4.0_f64) * t11037 * t18108 + F::cast_from(2.0_f64) * t14572 * t4669 + F::cast_from(4.0_f64) * t14618 * t14654 + F::cast_from(2.0_f64) * t14631 * t47853 + t3076 * t5941 + F::cast_from(2.0_f64) * t3189 * t63183 + t3204 * t5903 + t384 * t62604 + F::cast_from(4.0_f64) * t4615 * t4691;
    t63198
}
