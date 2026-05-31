//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1347/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1347<F: Float>(t11585: F, t4028: F, t11693: F, t51274: F, t14058: F, t3875: F, t36666: F, t850: F, t14093: F, t51412: F, t51415: F, t54330: F, t54345: F, t57171: F, t57174: F, t57176: F, t57179: F, t57182: F) -> F {
    let t57184 = t4028 * t11585;
    let t57186 = t51274 * t11693;
    let t57188 = t14058 * t3875;
    let t57190 = t850 * t36666;
    let t57191 = t57190 * t14093;
    let t57193 = -t57171 / F::cast_from(768.0_f64) - t57174 / F::cast_from(96.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t57176 + t57179 / F::cast_from(16.0_f64) - t54330 - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t51412 - t51415 - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t57182 - t57184 / F::cast_from(16.0_f64) - t57186 / F::cast_from(16.0_f64) - F::cast_from(35.0_f64) / F::cast_from(576.0_f64) * t57188 - t57191 / F::cast_from(96.0_f64) - t54345;
    t57193
}
