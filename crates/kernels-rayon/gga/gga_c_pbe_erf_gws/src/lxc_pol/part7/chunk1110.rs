//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1110/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1110(t2397: f64, t4424: f64, t2387: f64, t4423: f64, t833: f64, t2233: f64, t4442: f64, t4414: f64, t4493: f64, t2246: f64, t4433: f64, t6757: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19875 = t4424 * t2397;
    let t19878 = t2387 * t4423 * t833;
    let t19880 = t4442 * t2233;
    let t19888 = t4414 * t4493;
    let t19890 = t2246 * t4433;
    let t19892 = t4414 * t6757;
    (t19875, t19878, t19880, t19888, t19890, t19892)
}
