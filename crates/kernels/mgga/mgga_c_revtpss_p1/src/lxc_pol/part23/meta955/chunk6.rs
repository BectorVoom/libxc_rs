//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3188/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3188<F: Float>(t1266: F, t17290: F, t21085: F, t21137: F, t21140: F, t21213: F, t5313: F, t5327: F, t5373: F, t57727: F, t6647: F, t83719: F, t83725: F, t83728: F, t83731: F, t83735: F) -> F {
    let t83741 = t5373 * t21137 / F::cast_from(9.0_f64) + t5373 * t21140 / F::cast_from(6.0_f64) + t83719 / F::cast_from(216.0_f64) + F::cast_from(11.0_f64) / F::cast_from(81.0_f64) * t21213 * t5313 + F::cast_from(0.35400176935018568008e-1_f64) * t83725 * t1266 + F::cast_from(0.22866142996303859718e-2_f64) * t83728 * t1266 - t57727 - F::cast_from(0.14481890564325777821e-1_f64) * t83731 - F::cast_from(0.14291339372689912324e-3_f64) * t83735 - F::cast_from(0.64311027177104605458e-3_f64) * t17290 * t6647 - F::cast_from(0.64311027177104605458e-3_f64) * t5327 * t21085;
    t83741
}
