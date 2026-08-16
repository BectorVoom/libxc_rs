//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1116/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1116<F: Float>(t1349: F, t138551: F, t138557: F, t138560: F, t138652: F, t138655: F, t147590: F, t147602: F, t147604: F, t165: F, t23413: F, t26515: F, t26581: F, t27422: F, t28: F, t35007: F, t35234: F, t378: F, t525: F, t5772: F, t5845: F, t7309: F, t7313: F, t7315: F) -> F {
    let t147614 = -t138551 / F::cast_from(18.0_f64) + t35007 * t5845 / F::cast_from(6.0_f64) + t1349 * t28 * t525 * t147590 * t165 / F::cast_from(6.0_f64) + t138557 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5772 * t378 * t7313 * t27422 - t138560 / F::cast_from(18.0_f64) + t147602 / F::cast_from(54.0_f64) + t147604 / F::cast_from(9.0_f64) - t23413 * t35234 / F::cast_from(9.0_f64) + t7309 * t26515 / F::cast_from(6.0_f64) - t138652 / F::cast_from(18.0_f64) - t138655 / F::cast_from(9.0_f64) - t26581 * t7315 / F::cast_from(3.0_f64);
    t147614
}
