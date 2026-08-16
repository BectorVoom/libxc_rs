//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 834/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk834(t2729: f64, t586: f64, t2609: f64, t1037: f64, t5467: f64, t4913: f64, t213: f64, t331: f64, t34: f64, t649: f64, t661: f64, t1620: f64) -> (f64, f64, f64, f64, f64) {
    let t7011 = t2729 * t586;
    let t7013 = 8.0_f64 / 15.0_f64 * t7011 * t2609;
    let t7015 = 8.0_f64 / 45.0_f64 * t5467 * t1037;
    let t7017 = 8.0_f64 / 15.0_f64 * t4913 * t2609;
    let t7018 = t331 * t213;
    let t7019 = t649 * t34;
    let t7020 = t7019 * t661;
    let t7021 = t7018 * t7020;
    let t7023 = 8.0_f64 / 15.0_f64 * t1620 * t7021;
    (t7011, t7013, t7015, t7017, t7023)
}
