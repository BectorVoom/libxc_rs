//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 528/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk528(t2615: f64, t593: f64, t1010: f64, t1648: f64, t331: f64, t589: f64, t34: f64, t591: f64, t587: f64, t1017: f64, t597: f64, t562: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2617 = 4.0_f64 / 45.0_f64 * t2615 * t593;
    let t2619 = 4.0_f64 / 45.0_f64 * t1648 * t1010;
    let t2620 = t331 * t589;
    let t2621 = t591 * t34;
    let t2622 = t2620 * t2621;
    let t2624 = 8.0_f64 / 45.0_f64 * t587 * t2622;
    let t2625 = t597 * t1017;
    let t2626 = t2625 * t562;
    (t2617, t2619, t2620, t2621, t2622, t2624, t2626)
}
