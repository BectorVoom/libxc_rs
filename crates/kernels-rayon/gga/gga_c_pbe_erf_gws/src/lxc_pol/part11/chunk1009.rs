//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1009/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1009(t3443: f64, t995: f64, t2749: f64, t3479: f64, t12701: f64, t597: f64, t1802: f64, t10510: f64, t7130: f64, t10992: f64, t2615: f64, t12584: f64, t211: f64, t582: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40558 = t3443 * t995;
    let t40563 = t3479 * t2749;
    let t40566 = t597 * t12701;
    let t40571 = t1802 * t12701;
    let t40604 = t7130 * t10510;
    let t40655 = t2615 * t10992;
    let t40672 = t211 * t582 * t12584;
    (t40558, t40563, t40566, t40571, t40604, t40655, t40672)
}
