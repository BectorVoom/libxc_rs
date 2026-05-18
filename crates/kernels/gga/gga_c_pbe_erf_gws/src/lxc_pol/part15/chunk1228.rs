//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1228/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1228<F: Float>(t13772: F, t14397: F, t14437: F, t14791: F, t2388: F, t2392: F, t2408: F, t3040: F, t50927: F, t52940: F, t52944: F, t52952: F, t52956: F, t52959: F, t52962: F, t52969: F, t52971: F, t52973: F, t52976: F, t9218: F, t9283: F) -> F {
    let t52978 = -t2388 * t14437 / F::new(96.0) + t52940 / F::new(384.0) + t52944 / F::new(768.0) + t2408 * t9283 * t14791 * t9218 / F::new(8.0) - t52952 / F::new(3072.0) + t52956 / F::new(768.0) - t52959 / F::new(192.0) - t52962 + F::new(7.0) / F::new(1152.0) * t50927 - t3040 * t13772 / F::new(48.0) - t2392 * t14397 / F::new(96.0) + t52969 + t52971 + t52973 + t52976 / F::new(768.0);
    t52978
}
