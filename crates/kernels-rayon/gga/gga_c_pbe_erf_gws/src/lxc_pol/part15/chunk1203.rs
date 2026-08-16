//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1203/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1203(t2299: f64, t254: f64, t3970: f64, t13925: f64, t19777: f64, t13807: f64, t13916: f64, t13920: f64, t14791: f64, t2417: f64, t353: f64, t859: f64) -> (f64, f64, f64, f64, f64) {
    let t51555 = t3970 * t2299 * t254;
    let t51561 = t19777 * t13925;
    let t51563 = t13807 * t13916;
    let t51564 = t51563 * t13920;
    let t51569 = t859 * t353 * t14791 * t2417;
    (t51555, t51561, t51563, t51564, t51569)
}
