//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1231/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1231<F: Float>(t14423: F, t14682: F, t3989: F, t6360: F, t14711: F, t8801: F, t13888: F, t14651: F, t14791: F, t2408: F, t29751: F, t3066: F, t3207: F, t50944: F, t52982: F, t52986: F, t52989: F, t52992: F, t52994: F, t52997: F, t53009: F, t53012: F, t53015: F, t9213: F, t9283: F, t9321: F) -> F {
    let t53019 = t3989 * t14682 * t14423 * t6360;
    let t53025 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t8801 * t14711;
    let t53026 = -t52982 / F::cast_from(192.0_f64) - t52986 / F::cast_from(192.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t50944 + t52989 + t52992 - t52994 / F::cast_from(24.0_f64) - t52997 / F::cast_from(24.0_f64) + t3207 * t9283 * t13888 * t9213 / F::cast_from(8.0_f64) - t3066 * t9283 * t14791 * t9321 / F::cast_from(16.0_f64) - t53009 / F::cast_from(1536.0_f64) - t53012 + F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t53015 + t53019 / F::cast_from(1536.0_f64) - t2408 * t29751 * t14651 / F::cast_from(12.0_f64) + t53025;
    t53026
}
