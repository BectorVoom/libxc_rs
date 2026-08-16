//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1222/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1222(t13807: f64, t13916: f64, t2242: f64, t4055: f64, t2306: f64, t332: f64, t1477: f64, t326: f64, t886: f64, t3960: f64, t1176: f64, t2344: f64, t923: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51563 = t13807 * t13916;
    let t51572 = t2242 * t4055;
    let t51580 = t2306 * t332;
    let t51649 = t326 * t1477;
    let t51650 = t51649 * t886;
    let t51651 = t51650 * t3960;
    let t51666 = t1176 * t923 * t2344;
    (t51563, t51572, t51580, t51649, t51650, t51651, t51666)
}
