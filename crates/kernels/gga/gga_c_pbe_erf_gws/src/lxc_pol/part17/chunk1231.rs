//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1231/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1231<F: Float>(t14423: F, t14682: F, t3989: F, t6360: F, t14711: F, t8801: F, t13888: F, t14651: F, t14791: F, t2408: F, t29751: F, t3066: F, t3207: F, t50944: F, t52982: F, t52986: F, t52989: F, t52992: F, t52994: F, t52997: F, t53009: F, t53012: F, t53015: F, t9213: F, t9283: F, t9321: F) -> F {
    let t53019 = t3989 * t14682 * t14423 * t6360;
    let t53025 = F::new(7.0) / F::new(24.0) * t8801 * t14711;
    let t53026 = -t52982 / F::new(192.0) - t52986 / F::new(192.0) + F::new(7.0) / F::new(144.0) * t50944 + t52989 + t52992 - t52994 / F::new(24.0) - t52997 / F::new(24.0) + t3207 * t9283 * t13888 * t9213 / F::new(8.0) - t3066 * t9283 * t14791 * t9321 / F::new(16.0) - t53009 / F::new(1536.0) - t53012 + F::new(35.0) / F::new(432.0) * t53015 + t53019 / F::new(1536.0) - t2408 * t29751 * t14651 / F::new(12.0) + t53025;
    t53026
}
