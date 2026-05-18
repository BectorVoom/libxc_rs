//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1360/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1360<F: Float>(t15482: F, t2376: F, t829: F, t830: F, t11401: F, t14185: F, t14894: F, t14918: F, t14958: F, t2408: F, t2498: F, t3207: F, t35566: F, t55077: F, t55087: F, t56323: F, t56333: F, t56337: F, t56341: F, t56343: F, t56349: F, t56351: F, t56357: F, t827: F, t9283: F) -> F {
    let t58103 = t2376 * t15482;
    let t58105 = t829 * t830 * t58103;
    let t58110 = -F::new(7.0) / F::new(1152.0) * t56323 + t56333 / F::new(384.0) - t55077 + t56337 / F::new(192.0) + t56341 / F::new(192.0) + F::new(7.0) / F::new(2304.0) * t56343 + F::new(7.0) / F::new(2304.0) * t56349 + t56351 / F::new(48.0) - t3207 * t35566 * t14894 / F::new(8.0) + t55087 - t2408 * t9283 * t14185 * t11401 / F::new(12.0) - t2408 * t35566 * t14958 / F::new(12.0) - F::new(7.0) / F::new(288.0) * t56357 - t827 * t58105 / F::new(96.0) - t2498 * t14918 / F::new(48.0);
    t58110
}
