//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1331/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1331<F: Float>(t4142: F, t51529: F, t13953: F, t14648: F, t51877: F, t14404: F, t14710: F, t19895: F, t22142: F, t2220: F, t29751: F, t3189: F, t3207: F, t335: F, t338: F, t353: F, t4002: F, t4053: F, t4183: F, t51081: F, t51087: F, t51864: F, t51870: F, t51881: F, t51896: F, t54018: F, t54041: F, t54074: F, t54104: F, t54132: F, t54156: F, t54181: F, t54211: F, t54235: F, t54263: F, t54291: F, t54312: F, t54337: F, t54364: F, t54388: F, t54413: F, t8793: F, t898: F, t9283: F) -> F {
    let t54427 = t51529 * t4142;
    let t54429 = t13953 * t14648;
    let t54430 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t54429;
    let t54435 = F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t51877;
    let t54449 = -F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t51864 - t335 * t338 * t353 * t898 * (t54018 + t54041 + t54074 + t54104 + t54132 + t54156 + t54181 + t54211 + t54235 + t54263 + t54291 + t54312 + t54337 + t54364 + t54388 + t54413) / F::cast_from(96.0_f64) - t335 * t338 * t2220 * t4183 / F::cast_from(96.0_f64) - F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t54427 + t54430 - t51870 + t8793 * t51081 / F::cast_from(24.0_f64) + t8793 * t51087 / F::cast_from(24.0_f64) + t54435 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t51881 + t19895 * t14404 / F::cast_from(48.0_f64) - t22142 * t4002 / F::cast_from(96.0_f64) - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t51896 - t3207 * t29751 * t14710 / F::cast_from(8.0_f64) - t3207 * t9283 * t4053 * t3189 / F::cast_from(8.0_f64);
    t54449
}
