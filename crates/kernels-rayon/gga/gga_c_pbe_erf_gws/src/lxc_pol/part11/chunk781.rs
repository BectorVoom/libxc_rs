//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 781/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk781(t12464: f64, t1856: f64, t12468: f64, t12460: f64, t5264: f64, t12480: f64, t606: f64, t10823: f64, t10825: f64, t10827: f64, t25: f64, t5241: f64, t5271: f64, t7374: f64, t7407: f64) -> (f64, f64, f64, f64, f64) {
    let t12683 = t1856 * t12464;
    let t12686 = t1856 * t12468;
    let t12693 = t5264 * t12460;
    let t12696 = t606 * t12480;
    let t12700 = 0.13333333333333333333e-1_f64 * t25 * t12683 - 0.66666666666666666666e-2_f64 * t25 * t12686 - t5241 + 0.35991666666666666666e-1_f64 * t10827 - 0.22222222222222222222e-1_f64 * t7407 + 0.23994444444444444444e-1_f64 * t10823 - 0.71983333333333333333e-1_f64 * t10825 - 0.29629629629629629629e-2_f64 * t25 * t12693 - 0.66666666666666666667e-2_f64 * t25 * t12696 - t5271 - 0.47988888888888888888e-1_f64 * t7374;
    (t12683, t12686, t12693, t12696, t12700)
}
