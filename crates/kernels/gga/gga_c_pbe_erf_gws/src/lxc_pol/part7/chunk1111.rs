//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1111/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1111<F: Float>(t2306: F, t4383: F, t2382: F, t4395: F, t19875: F, t19878: F, t19880: F, t19888: F, t19890: F, t19892: F, t2373: F, t2408: F, t2409: F, t4390: F, t4397: F, t4459: F, t4464: F, t4484: F, t6112: F, t6138: F, t6797: F, t8734: F) -> (F, F) {
    let t19894 = t2306 * t4383;
    let t19895 = t2382 * t19894;
    let t19898 = t4395 * t4383;
    let t19899 = t2382 * t19898;
    let t19904 = -t2408 * t2409 * t8734 * t6138 / F::cast_from(2.0_f64) + F::cast_from(35.0_f64) / F::cast_from(36.0_f64) * t19875 + F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t19878 - F::cast_from(35.0_f64) / F::cast_from(36.0_f64) * t19880 - t6112 * t2373 / F::cast_from(12.0_f64) - t4397 * t4459 / F::cast_from(8.0_f64) - t4397 * t4464 / F::cast_from(24.0_f64) - F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t19888 + F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t19890 - F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t19892 + t19895 * t6797 / F::cast_from(4.0_f64) + t19899 * t4390 / F::cast_from(4.0_f64) + t19899 * t4484 / F::cast_from(8.0_f64);
    (t19894, t19904)
}
