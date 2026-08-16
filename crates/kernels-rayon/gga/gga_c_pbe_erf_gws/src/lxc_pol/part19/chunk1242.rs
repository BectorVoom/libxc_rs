//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1242/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1242(t21296: f64, t367: f64, t899: f64, t3237: f64, t51371: f64, t3242: f64, t3232: f64, t51388: f64, t51396: f64, t14079: f64, t3283: f64, t1154: f64, t51387: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t54279 = t899 * t21296 * t367;
    let t54283 = t51371 * t3237;
    let t54285 = t51371 * t3242;
    let t54289 = t51371 * t3232;
    let t54293 = 119.0_f64 / 1728.0_f64 * t51388;
    let t54294 = 119.0_f64 / 864.0_f64 * t51396;
    let t54301 = t14079 * t3283;
    let t54305 = t51387 * t1154;
    (t54279, t54283, t54285, t54289, t54293, t54294, t54301, t54305)
}
