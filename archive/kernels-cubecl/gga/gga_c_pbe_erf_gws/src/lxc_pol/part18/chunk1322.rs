//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1322/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1322<F: Float>(t11516: F, t14011: F, t11934: F, t51222: F, t54053: F, t54073: F, t54088: F, t55469: F, t56910: F, t56912: F, t56914: F, t56917: F, t56920: F, t56922: F) -> F {
    let t56924 = t14011 * t11516;
    let t56926 = t14011 * t11934;
    let t56928 = F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t51222 + t56910 / F::cast_from(48.0_f64) - t54053 + t56912 / F::cast_from(192.0_f64) + t56914 / F::cast_from(24.0_f64) + t54073 + t56917 / F::cast_from(48.0_f64) - t56920 / F::cast_from(96.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t56922 + t54088 + t55469 + t56924 / F::cast_from(192.0_f64) - t56926 / F::cast_from(768.0_f64);
    t56928
}
