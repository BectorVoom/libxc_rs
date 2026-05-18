//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1123/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1123<F: Float>(t14136: F, t14138: F, t1173: F, t2222: F, t13977: F, t13981: F, t13985: F, t13989: F, t13991: F, t13996: F, t13999: F, t14003: F, t14109: F, t14115: F, t14119: F, t14123: F, t14129: F, t14131: F, t14133: F, t2408: F, t3066: F, t335: F) -> F {
    let t14139 = t14136 * t14138;
    let t14141 = t1173 * t2222;
    let t14143 = -t13977 / F::new(96.0) + t2408 * t13981 / F::new(48.0) - t13985 / F::new(48.0) + t13989 + t3066 * t13991 / F::new(24.0) + t3066 * t13996 / F::new(24.0) - F::new(7.0) / F::new(72.0) * t13999 + t14003 - t335 * t14109 / F::new(96.0) + t14115 + t14119 / F::new(1536.0) + t14123 / F::new(16.0) - t14129 - t14131 - t14133 / F::new(1536.0) - t14139 / F::new(96.0) + t14141 / F::new(96.0);
    t14143
}
