//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 928/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk928<F: Float>(t120: F, t133: F, t4573: F, t1473: F, t1497: F, t5615: F, t751: F, t1332: F, t296: F, t6073: F, t2059: F, t2060: F, t279: F, t6045: F) -> (F, F, F, F, F) {
    let t19439 = F::new(0.29801938271604938271e1) * t133 * t4573 * t120;
    let t19458 = F::new(0.31931290694012290916e0) * t1473 * t1497;
    let t19466 = F::new(0.79828226735030727292e-1) * t751 * t5615;
    let t19482 = F::new(0.47400060215270560269e1) * t6073 * t1332 * t296;
    let t19517 = F::new(0.16521134411652656606e2) * t2059 * t2060 * t6045 * t279;
    (t19439, t19458, t19466, t19482, t19517)
}
