//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3084/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3084<F: Float>(t56183: F, t56236: F, t58536: F, t68389: F, t68399: F, t81224: F, t81228: F, t81230: F, t81232: F, t81234: F, t81236: F, t44919: F, t52011: F, t77513: F) -> (F, F) {
    let t81485 = F::cast_from(0.79724444444444444446e0_f64) * t56183 + F::cast_from(0.53814e1_f64) * t81224 + F::cast_from(0.29896666666666666667e0_f64) * t81228 - F::cast_from(0.11072839506172839506e0_f64) * t81230 + F::cast_from(0.39862222222222222223e0_f64) * t81232 - F::cast_from(0.59793333333333333333e0_f64) * t81234 - F::cast_from(0.99655555555555555557e-1_f64) * t81236 + t58536 - F::cast_from(0.93011851851851851854e0_f64) * t56236 - F::cast_from(0.29896666666666666667e0_f64) * t68389 + F::cast_from(0.79724444444444444444e0_f64) * t68399;
    let t81489 = t52011 * t44919 * t77513;
    (t81485, t81489)
}
