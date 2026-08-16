//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1078/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1078(t12041: f64, t2383: f64, t3037: f64, t829: f64, t830: f64, t831: f64, t1105: f64, t2501: f64, t2370: f64, t1115: f64, t11409: f64, t12101: f64, t12111: f64, t12121: f64, t12125: f64, t2498: f64, t2503: f64, t3040: f64, t3047: f64, t3052: f64, t3066: f64, t335: f64, t827: f64, t844: f64, t8584: f64, t8592: f64, t8818: f64, t9718: f64, t9723: f64) -> (f64, f64) {
    let t12130 = t12041 * t2383;
    let t12133 = t829 * t830 * t831 * t3037;
    let t12136 = t2501 * t1105;
    let t12138 = t2370 * t830 * t12136;
    let t12147 = t3066 * t11409 / 24.0_f64 - t335 * t12101 / 96.0_f64 - 35.0_f64 / 216.0_f64 * t8818 - t1115 * t8592 / 48.0_f64 - t1115 * t9723 / 24.0_f64 + t827 * t12111 / 48.0_f64 - t3040 * t3052 / 24.0_f64 - t2498 * t3052 / 24.0_f64 - t1115 * t9718 / 24.0_f64 - t844 * t12121 / 24.0_f64 - t844 * t12125 / 24.0_f64 + t3040 * t2503 / 48.0_f64 + t12130 * t12133 / 48.0_f64 - t827 * t12138 / 24.0_f64 - t3040 * t3047 / 48.0_f64 - t2498 * t3047 / 48.0_f64 - t1115 * t8584 / 48.0_f64;
    (t12136, t12147)
}
