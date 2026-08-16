//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 911/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk911(t1648: f64, t4992: f64, t4995: f64, t1678: f64, t1773: f64, t184: f64, t199: f64, t5130: f64, t17128: f64, t17133: f64, t17138: f64, t17141: f64, t17144: f64, t17147: f64, t17150: f64) -> (f64, f64, f64, f64, f64) {
    let t17151 = t1648 * t4992;
    let t17152 = 32.0_f64 / 135.0_f64 * t17151;
    let t17153 = t1648 * t4995;
    let t17154 = 64.0_f64 / 45.0_f64 * t17153;
    let t17158 = 8.0_f64 / 5.0_f64 * t1678 * t1773 * t184 * t199;
    let t17159 = t1648 * t5130;
    let t17160 = 64.0_f64 / 45.0_f64 * t17159;
    let t17161 = -t17128 + t17133 + t17138 - t17141 - t17144 - t17147 + t17150 - t17152 - t17154 + t17158 - t17160;
    (t17152, t17154, t17158, t17160, t17161)
}
