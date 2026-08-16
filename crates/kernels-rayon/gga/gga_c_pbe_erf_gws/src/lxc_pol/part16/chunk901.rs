//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 901/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk901(t211: f64, t7844: f64, t2826: f64, t612: f64, t1006: f64, t1868: f64, t1798: f64, t2741: f64, t219: f64, t5400: f64, t7283: f64, t639: f64) -> (f64, f64, f64, f64, f64) {
    let t7845 = t211 * t7844;
    let t7846 = 4.0_f64 / 135.0_f64 * t7845;
    let t7848 = 4.0_f64 / 15.0_f64 * t2826 * t612;
    let t7850 = 2.0_f64 / 15.0_f64 * t1006 * t1868;
    let t7852 = 16.0_f64 / 45.0_f64 * t2741 * t1798;
    let t7853 = t5400 * t219;
    let t7854 = t7853 * t7283;
    let t7856 = 32.0_f64 / 81.0_f64 * t639 * t7854;
    (t7846, t7848, t7850, t7852, t7856)
}
