//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 947/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk947(t10676: f64, t5218: f64, t572: f64, t7514: f64, t10392: f64, t610: f64, t7062: f64, t1651: f64, t3503: f64, t587: f64, t2609: f64, t7527: f64) -> (f64, f64, f64, f64) {
    let t10678 = 16.0_f64 / 45.0_f64 * t5218 * t10676;
    let t10679 = t7514 * t572;
    let t10681 = t10679 * t10392 * t610;
    let t10683 = 16.0_f64 / 45.0_f64 * t7062 * t10681;
    let t10685 = t1651 * t3503;
    let t10686 = t587 * t10685;
    let t10687 = 16.0_f64 / 135.0_f64 * t10686;
    let t10690 = 8.0_f64 / 15.0_f64 * t7527 * t2609;
    (t10678, t10683, t10687, t10690)
}
