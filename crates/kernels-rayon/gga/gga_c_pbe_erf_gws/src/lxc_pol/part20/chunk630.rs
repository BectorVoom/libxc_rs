//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 630/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk630(t2507: f64, t2841: f64, t2843: f64, t2845: f64, t3360: f64, t85: f64, t2516: f64, t1267: f64, t1288: f64, t1296: f64, t1446: f64, t1450: f64, t3362: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3365 = 0.36623110073506319882e-3_f64 * t2507;
    let t3366 = 0.11696446794910408142e1_f64 * t2841;
    let t3367 = 8.0_f64 * t2843;
    let t3368 = 8.0_f64 * t2845;
    let t3369 = t3360 * t85;
    let t3370 = 0.19751789702565206229e-1_f64 * t3369;
    let t3371 = 2.0_f64 * t2516;
    let t3372 = -t1267 + t1288 + t1296 + t1446 + t1450 - t3365 - t3366 - t3367 - t3368 + t3370 + t3371 + t3362;
    (t3365, t3366, t3367, t3368, t3370, t3371, t3372)
}
