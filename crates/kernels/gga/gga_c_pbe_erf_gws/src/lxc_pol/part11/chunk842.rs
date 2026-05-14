//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 842/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk842<F: Float>(t1332: F, t296: F, t6073: F, t2059: F, t2060: F, t279: F, t6045: F, t116: F, t366: F, t798: F, t799: F, t311: F, t19: F, t2331: F, t301: F, t305: F) -> (F, F, F, F) {
    let t19482 = 0.47400060215270560269e1 * t6073 * t1332 * t296;
    let t19517 = 0.16521134411652656606e2 * t2059 * t2060 * t6045 * t279;
    let t19525 = 0.6693920255418271605e1 * t798 * t799 * t366 * t116;
    let t19530 = t311 * t311;
    let t19537 = 0.34072858057724757727e0 * t305 / t19530 * t2331 * t301 * t19 * t799;
    (t19482, t19517, t19525, t19537)
}
