//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 848/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk848(t13368: f64, t2157: f64, t858: f64, t867: f64, t2155: f64, t13187: f64, t2210: f64, t884: f64, t3219: f64, t3235: f64, t3855: f64, t11603: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13369 = t13368 * t2157;
    let t13371 = t867 * t858 * t13369;
    let t13373 = t2155 * t13371 / 16.0_f64;
    let t13375 = t2210 * t858 * t13187;
    let t13377 = 3.0_f64 / 16.0_f64 * t884 * t13375;
    let t13379 = t3235 * t3219 * t3855;
    let t13384 = 7.0_f64 / 48.0_f64 * t11603;
    (t13369, t13371, t13373, t13375, t13377, t13379, t13384)
}
