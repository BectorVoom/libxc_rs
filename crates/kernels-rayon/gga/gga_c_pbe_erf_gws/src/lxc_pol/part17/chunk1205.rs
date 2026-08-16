//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1205/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1205(t353: f64, t4053: f64, t4386: f64, t810: f64, t1193: f64, t2074: f64, t1477: f64, t326: f64, t886: f64, t3960: f64, t1176: f64, t2344: f64, t923: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51599 = t4386 * t353 * t4053 * t810;
    let t51604 = t4386 * t353 * t1193 * t2074;
    let t51649 = t326 * t1477;
    let t51650 = t51649 * t886;
    let t51651 = t51650 * t3960;
    let t51666 = t1176 * t923 * t2344;
    (t51599, t51604, t51649, t51650, t51651, t51666)
}
