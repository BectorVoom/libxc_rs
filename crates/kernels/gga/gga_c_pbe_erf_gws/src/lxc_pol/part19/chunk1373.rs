//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1373/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1373<F: Float>(t15425: F, t4414: F, t1205: F, t12098: F, t12213: F, t14935: F, t15443: F, t2409: F, t3066: F, t3067: F, t53886: F, t55382: F, t55385: F, t55403: F, t55420: F, t55421: F, t57260: F, t57262: F, t57265: F, t57284: F, t57287: F, t57289: F, t8734: F) -> F {
    let t58479 = t4414 * t15425;
    let t58488 = t3066 * t2409 * t12213 * t14935 / F::cast_from(24.0_f64) + t3066 * t2409 * t8734 * t15443 / F::cast_from(24.0_f64) + t3066 * t2409 * t3067 * t1205 * t12098 / F::cast_from(48.0_f64) - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t58479 - t55382 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t57260 + t57262 / F::cast_from(12.0_f64) + t55385 + t57265 / F::cast_from(24.0_f64) + t55403 + F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t53886 + t57284 / F::cast_from(768.0_f64) + t57287 / F::cast_from(768.0_f64) - t57289 / F::cast_from(24.0_f64) + t55420 - t55421;
    t58488
}
