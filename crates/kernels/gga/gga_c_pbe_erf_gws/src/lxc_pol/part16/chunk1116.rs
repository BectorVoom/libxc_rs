//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1116/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1116<F: Float>(t14222: F, t14239: F, t898: F, t338: F, t353: F, t1205: F, t2182: F, t2376: F, t2409: F, t13778: F, t13785: F, t13789: F, t13794: F, t13801: F, t13804: F, t13809: F, t13813: F, t13818: F, t14182: F, t14188: F, t14193: F, t14198: F, t14202: F, t2384: F, t2408: F, t3207: F, t335: F, t4083: F, t4385: F, t6793: F) -> (F, F, F, F, F, F) {
    let t14240 = t14222 + t14239;
    let t14241 = t898 * t14240;
    let t14243 = t338 * t353 * t14241;
    let t14250 = t1205 * t2182;
    let t14252 = t2409 * t2376 * t14250;
    let t14257 = -t13778 / F::cast_from(96.0_f64) - t13785 / F::cast_from(384.0_f64) + t6793 * t14182 / F::cast_from(24.0_f64) + t6793 * t14188 / F::cast_from(24.0_f64) + t4385 * t14193 / F::cast_from(96.0_f64) - t2384 * t4083 / F::cast_from(96.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t14198 + t2408 * t14202 / F::cast_from(48.0_f64) - t13789 / F::cast_from(1536.0_f64) - t335 * t14243 / F::cast_from(96.0_f64) - t13794 / F::cast_from(12.0_f64) + t13801 / F::cast_from(768.0_f64) + t13804 / F::cast_from(768.0_f64) - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t13809 - t3207 * t14252 / F::cast_from(16.0_f64) + t13813 / F::cast_from(48.0_f64) + t13818 / F::cast_from(48.0_f64);
    (t14240, t14241, t14243, t14250, t14252, t14257)
}
