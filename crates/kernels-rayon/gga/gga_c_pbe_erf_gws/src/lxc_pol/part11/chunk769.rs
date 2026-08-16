//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 769/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk769(t12549: f64, t590: f64, t587: f64, t2615: f64, t3531: f64, t12339: f64, t5294: f64, t5293: f64, t10378: f64, t995: f64, t1885: f64, t1820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12550 = t590 * t12549;
    let t12552 = 4.0_f64 / 45.0_f64 * t587 * t12550;
    let t12554 = 4.0_f64 / 9.0_f64 * t2615 * t3531;
    let t12555 = t5294 * t12339;
    let t12556 = t5293 * t12555;
    let t12558 = 32.0_f64 / 81.0_f64 * t587 * t12556;
    let t12559 = t10378 * t995;
    let t12560 = t1885 * t12559;
    let t12562 = 8.0_f64 / 5.0_f64 * t1820 * t12560;
    (t12550, t12552, t12554, t12555, t12556, t12558, t12559, t12560, t12562)
}
