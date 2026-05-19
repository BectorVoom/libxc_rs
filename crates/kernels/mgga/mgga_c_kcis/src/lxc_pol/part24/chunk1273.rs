//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1273/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1273<F: Float>(t28190: F, t28214: F, t100814: F, t100817: F, t100820: F, t100823: F, t26966: F, t29104: F, t8091: F, t96412: F, t97010: F, t97442: F, t97449: F, t97465: F) -> F {
    let t100830 = t28190 * t28214;
    let t100832 = F::cast_from(0.10317654320987654321e-2_f64) * t100814 + F::cast_from(0.92858888888888888885e-2_f64) * t100817 + F::cast_from(0.61905925925925925925e-2_f64) * t100820 - F::cast_from(0.41270617283950617283e-2_f64) * t100823 - F::cast_from(0.61782407407407407408e-3_f64) * t26966 * t29104 + t97442 - t97449 - F::cast_from(0.51588271604938271603e-3_f64) * t96412 + t97465 + F::cast_from(0.61782407407407407408e-3_f64) * t97010 * t8091 - F::cast_from(0.7722800925925925926e-4_f64) * t100830;
    t100832
}
