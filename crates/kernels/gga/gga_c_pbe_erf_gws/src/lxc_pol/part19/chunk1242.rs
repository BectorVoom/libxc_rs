//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1242/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1242<F: Float>(t21296: F, t367: F, t899: F, t3237: F, t51371: F, t3242: F, t3232: F, t51388: F, t51396: F, t14079: F, t3283: F, t1154: F, t51387: F) -> (F, F, F, F, F, F, F, F) {
    let t54279 = t899 * t21296 * t367;
    let t54283 = t51371 * t3237;
    let t54285 = t51371 * t3242;
    let t54289 = t51371 * t3232;
    let t54293 = F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t51388;
    let t54294 = F::cast_from(119.0_f64) / F::cast_from(864.0_f64) * t51396;
    let t54301 = t14079 * t3283;
    let t54305 = t51387 * t1154;
    (t54279, t54283, t54285, t54289, t54293, t54294, t54301, t54305)
}
