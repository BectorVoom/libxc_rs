//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 879/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk879(t1802: f64, t2784: f64, t610: f64, t1885: f64, t587: f64, t1635: f64, t2612: f64, t1645: f64, t1656: f64, t2615: f64, t1666: f64, t1010: f64, t5406: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7589 = t1802 * t2784;
    let t7590 = t7589 * t610;
    let t7591 = t1885 * t7590;
    let t7593 = 8.0_f64 / 15.0_f64 * t587 * t7591;
    let t7595 = 4.0_f64 / 45.0_f64 * t2612 * t1635;
    let t7597 = 4.0_f64 / 27.0_f64 * t2612 * t1645;
    let t7599 = 4.0_f64 / 45.0_f64 * t2615 * t1656;
    let t7601 = 4.0_f64 / 27.0_f64 * t2615 * t1666;
    let t7603 = 4.0_f64 / 45.0_f64 * t5406 * t1010;
    (t7593, t7595, t7597, t7599, t7601, t7603)
}
