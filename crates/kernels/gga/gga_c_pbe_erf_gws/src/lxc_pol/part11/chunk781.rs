//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 781/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk781<F: Float>(t13368: F, t2157: F, t858: F, t867: F, t2155: F, t13187: F, t2210: F, t884: F, t3219: F, t3235: F, t3855: F, t11603: F, t1076: F, t1105: F, t1123: F, t2255: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13369 = t13368 * t2157;
    let t13371 = t867 * t858 * t13369;
    let t13373 = t2155 * t13371 / 16.0;
    let t13375 = t2210 * t858 * t13187;
    let t13377 = 3.0 / 16.0 * t884 * t13375;
    let t13379 = t3235 * t3219 * t3855;
    let t13384 = 7.0 / 48.0 * t11603;
    let t13385 = t1076 * t1105;
    let t13387 = t2255 * t1123 * t13385;
    (t13369, t13371, t13373, t13375, t13377, t13379, t13384, t13385, t13387)
}
