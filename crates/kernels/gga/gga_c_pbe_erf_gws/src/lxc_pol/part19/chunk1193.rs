//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1193/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1193<F: Float>(t15543: F, t2376: F, t2409: F, t12213: F, t4216: F, t14185: F, t3742: F, t9283: F, t1115: F, t14338: F, t14918: F, t15343: F, t15346: F, t15348: F, t15358: F, t15367: F, t15372: F, t15374: F, t15378: F, t15528: F, t15532: F, t15537: F, t2408: F, t3066: F, t335: F, t3913: F, t4083: F, t8629: F) -> (F, F, F, F, F) {
    let t15545 = t2409 * t2376 * t15543;
    let t15550 = t2409 * t12213 * t4216;
    let t15558 = t14185 * t3742;
    let t15559 = t9283 * t15558;
    let t15565 = t14338 + t3066 * t15528 / F::new(48.0) - t335 * t15532 / F::new(48.0) + t8629 * t15537 / F::new(96.0) - t15343 / F::new(48.0) - t15346 / F::new(24.0) - t15348 / F::new(12.0) + t2408 * t15545 / F::new(48.0) - t15358 / F::new(1536.0) + t3066 * t15550 / F::new(24.0) - t15367 / F::new(1536.0) - t1115 * t14918 / F::new(48.0) - t3913 * t4083 / F::new(96.0) - t2408 * t15559 / F::new(12.0) + t15372 / F::new(768.0) + t15374 / F::new(48.0) + t15378 / F::new(48.0);
    (t15545, t15550, t15558, t15559, t15565)
}
