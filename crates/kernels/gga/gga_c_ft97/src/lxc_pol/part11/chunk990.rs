//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 990/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk990<F: Float>(t165: F, t39641: F, t39646: F, t39649: F, t39655: F, t39658: F, t40517: F, t40519: F, t40522: F, t40525: F, t40540: F, t40555: F, t40570: F, t40585: F, t515: F, t564: F, t9460: F) -> F {
    let t40590 = -F::cast_from(12.0_f64) * t39641 - F::cast_from(4.0_f64) * t564 * t9460 + F::cast_from(16.0_f64) * t39646 + F::cast_from(12.0_f64) * t39649 + F::cast_from(48.0_f64) * t39655 - F::cast_from(72.0_f64) * t39658 - F::cast_from(2.0_f64) * t40517 - F::cast_from(8.0_f64) * t40519 - F::cast_from(8.0_f64) * t40522 + F::cast_from(24.0_f64) * t40525 - t515 * (t40540 + t40555 + t40570 + t40585) * t165;
    t40590
}
