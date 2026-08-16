//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 968/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk968(t10930: f64, t1620: f64, t2612: f64, t2640: f64, t2684: f64, t7106: f64, t5211: f64, t3443: f64, t572: f64, t418: f64, t1827: f64, t587: f64) -> (f64, f64, f64, f64) {
    let t10931 = t1620 * t10930;
    let t10932 = 16.0_f64 / 45.0_f64 * t10931;
    let t10933 = t2612 * t2640;
    let t10934 = 16.0_f64 / 135.0_f64 * t10933;
    let t10935 = t7106 * t2684;
    let t10937 = 16.0_f64 / 45.0_f64 * t5211 * t10935;
    let t10938 = t3443 * t572;
    let t10939 = t10938 * t418;
    let t10940 = t1827 * t10939;
    let t10942 = 4.0_f64 / 45.0_f64 * t587 * t10940;
    (t10932, t10934, t10937, t10942)
}
