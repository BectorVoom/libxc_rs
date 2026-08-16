//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1341/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1341(t11541: f64, t13917: f64, t57591: f64, t3959: f64, t9869: f64, t2409: f64, t35023: f64, t3965: f64, t13776: f64, t3861: f64, t3975: f64, t9504: f64) -> (f64, f64, f64, f64) {
    let t57593 = t13917 * t57591 * t11541;
    let t57595 = t3959 * t9869;
    let t57598 = t3965 * t2409 * t35023;
    let t57602 = t13776 * t3975 * t3861 * t9504;
    (t57593, t57595, t57598, t57602)
}
