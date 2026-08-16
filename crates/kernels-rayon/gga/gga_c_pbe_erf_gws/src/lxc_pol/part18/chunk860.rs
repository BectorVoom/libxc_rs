//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 860/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk860(t833: f64, t8746: f64, t3199: f64, t376: f64, t829: f64, t830: f64, t3062: f64, t4414: f64, t4395: f64, t8652: f64, t3074: f64, t2379: f64, t3083: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8747 = t8746 * t833;
    let t8749 = t3199 * t376;
    let t8751 = t829 * t830 * t8749;
    let t8771 = 7.0_f64 / 72.0_f64 * t4414 * t3062;
    let t8775 = t4395 * t8652;
    let t8776 = t3074 * t8775;
    let t8780 = 7.0_f64 / 144.0_f64 * t3083 * t2379;
    (t8747, t8749, t8751, t8771, t8776, t8780)
}
