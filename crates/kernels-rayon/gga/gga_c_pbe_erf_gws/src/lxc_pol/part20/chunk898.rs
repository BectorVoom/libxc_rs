//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 898/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk898(t10007: f64, t4545: f64, t6968: f64, t8520: f64, t7986: f64, t4341: f64, t4349: f64, t4503: f64, t4506: f64, t4513: f64, t4539: f64, t4542: f64, t6918: f64, t6923: f64, t6932: f64, t7984: f64, t9764: f64, t9765: f64) -> (f64, f64, f64, f64) {
    let t10008 = 0.19751789702565206229e-1_f64 * t10007;
    let t10009 = 0.63272429661648472106e0_f64 * t4545;
    let t10010 = 0.21687161765563048429e-1_f64 * t6968;
    let t10011 = 0.12654485932329694421e1_f64 * t8520;
    let t10012 = 40.0_f64 * t7986;
    let t10013 = -t9764 + t9765 + t4341 - t4349 - t6918 + t4503 - t4506 - t4513 + t4539 - t6923 + t4542 + t10008 - t10009 + t6932 + t10010 - t7984 - t10011 + t10012;
    (t10008, t10010, t10012, t10013)
}
