//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 904/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk904(t639: f64, t7874: f64, t219: f64, t5480: f64, t2679: f64, t1027: f64, t1724: f64, t1815: f64, t1809: f64, t7264: f64, t2580: f64, t5125: f64) -> (f64, f64, f64, f64, f64) {
    let t7876 = 32.0_f64 / 135.0_f64 * t639 * t7874;
    let t7877 = t5480 * t219;
    let t7878 = t7877 * t2679;
    let t7880 = 16.0_f64 / 81.0_f64 * t639 * t7878;
    let t7881 = t1027 * t1724;
    let t7882 = t1815 * t7881;
    let t7884 = 4.0_f64 / 45.0_f64 * t639 * t7882;
    let t7885 = t1809 * t7264;
    let t7887 = 8.0_f64 / 45.0_f64 * t639 * t7885;
    let t7888 = t5125 * t2580;
    (t7876, t7880, t7884, t7887, t7888)
}
