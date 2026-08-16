//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 826/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk826(t6781: f64, t938: f64, t829: f64, t830: f64, t2074: f64, t831: f64, t2370: f64, t4383: f64, t824: f64, t822: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6782 = t6781 * t938;
    let t6784 = t829 * t830 * t6782;
    let t6787 = t831 * t2074;
    let t6789 = t2370 * t830 * t6787;
    let t6792 = t824 * t4383;
    let t6793 = t822 * t6792;
    (t6782, t6784, t6787, t6789, t6792, t6793)
}
