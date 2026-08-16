//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1066/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1066<F: Float>(t1286: F, t136116: F, t137476: F, t137497: F, t144765: F, t145705: F, t145719: F, t145731: F, t25523: F, t26128: F, t28: F, t32011: F, t32338: F, t3266: F, t34354: F, t34565: F, t34568: F, t492: F, t5495: F, t5501: F, t6421: F, t8411: F, t8418: F) -> F {
    let t145733 = t137476 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) * t144765 - F::cast_from(2.0_f64) * t145705 + t1286 * t28 * t32338 * t25523 + t1286 * t28 * t32338 * t26128 - F::cast_from(24.0_f64) * t8418 * t34565 * t492 - F::cast_from(12.0_f64) * t8418 * t34568 * t492 + F::cast_from(8.0_f64) * t145719 - t137497 / F::cast_from(18.0_f64) + t5501 * t8411 * t32011 * t3266 - t1286 * t28 * t136116 * t6421 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t5495 * t34354 - t145731 / F::cast_from(9.0_f64);
    t145733
}
