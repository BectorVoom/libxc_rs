//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 680/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk680(t5463: f64, t644: f64, t639: f64, t1782: f64, t586: f64) -> (f64, f64, f64) {
    let t5464 = t5463 * t644;
    let t5465 = t639 * t5464;
    let t5466 = 8.0_f64 / 135.0_f64 * t5465;
    let t5467 = t1782 * t586;
    (t5464, t5466, t5467)
}
