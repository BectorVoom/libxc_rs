//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 665/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk665(t2416: f64, t3721: f64, t353: f64, t338: f64, t1109: f64, t830: f64, t831: f64, t829: f64) -> (f64, f64, f64, f64) {
    let t3722 = t2416 * t3721;
    let t3723 = t353 * t3722;
    let t3724 = t338 * t3723;
    let t3732 = t830 * t831 * t1109;
    let t3733 = t829 * t3732;
    (t3722, t3723, t3724, t3733)
}
