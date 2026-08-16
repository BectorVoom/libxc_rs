//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 674/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk674(t26768: f64, t525: f64, t165: f64, t28: f64, t3565: f64, t5935: f64, t1360: f64, t3588: f64, t1058: f64, t1359: f64, t1969: f64, t379: f64) -> (f64, f64, f64, f64, f64) {
    let t26769 = t525 * t26768;
    let t26770 = t26769 * t165;
    let t26771 = t28 * t26770;
    let t26777 = t5935 * t3565;
    let t26779 = t1360 * t3588;
    let t26780 = t28 * t26779;
    let t26783 = t1359 * t1058;
    let t26785 = t1969 * t26783 * t379;
    (t26769, t26771, t26777, t26780, t26785)
}
