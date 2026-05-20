//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3195/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3195<F: Float>(t17605: F, t21090: F, t127: F, t12988: F, t24617: F, t371: F, t20842: F, t5323: F, t12784: F, t12787: F, t12866: F, t17729: F, t21182: F, t24744: F, t24804: F, t44561: F, t44797: F, t5046: F, t59062: F, t6639: F, t71278: F, t71294: F, t71297: F) -> F {
    let t83916 = t17605 * t21090;
    let t83920 = t12988 * t371 * t127 * t24617;
    let t83922 = t5323 * t20842;
    let t83938 = F::cast_from(0.30488190661738479624e-2_f64) * t83916 - F::cast_from(0.85748036236139473947e-3_f64) * t83920 + F::cast_from(0.22866142996303859718e-2_f64) * t83922 + F::cast_from(0.7145669686344956162e-3_f64) * t12784 * t24804 - F::cast_from(0.71456696863449561621e-3_f64) * t17729 * t12787 * t5046 * t21182 + F::cast_from(0.45732285992607719436e-2_f64) * t71278 - t44797 + F::cast_from(0.28582678745379824648e-3_f64) * t71294 - t71297 / F::new(144.0) + F::cast_from(0.85748036236139473944e-3_f64) * t44561 * t24744 + F::cast_from(0.85748036236139473944e-3_f64) * t12866 * t59062 * t6639;
    t83938
}
