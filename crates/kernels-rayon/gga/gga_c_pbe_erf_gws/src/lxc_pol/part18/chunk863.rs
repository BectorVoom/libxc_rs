//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 863/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk863(t2134: f64, t8824: f64, t1133: f64, t874: f64, t3179: f64, t6331: f64, t2146: f64, t3165: f64, t5: f64, t2142: f64, t3108: f64, t3106: f64, t4395: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8826 = 7.0_f64 / 144.0_f64 * t2134 * t8824;
    let t8827 = t1133 * t874;
    let t8833 = t6331 * t3179;
    let t8835 = 7.0_f64 / 72.0_f64 * t2146 * t8833;
    let t8840 = t5 * t3165;
    let t8846 = 7.0_f64 / 144.0_f64 * t3108 * t2142;
    let t8847 = t4395 * t3106;
    (t8826, t8827, t8833, t8835, t8840, t8846, t8847)
}
