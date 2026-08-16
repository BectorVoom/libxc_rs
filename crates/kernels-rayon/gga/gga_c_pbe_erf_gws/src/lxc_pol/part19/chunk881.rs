//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 881/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk881(t829: f64, t830: f64, t9707: f64, t1105: f64, t2395: f64, t2370: f64, t2494: f64, t831: f64, t2358: f64, t3039: f64, t1114: f64, t4409: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9709 = t829 * t830 * t9707;
    let t9716 = t2395 * t1105;
    let t9718 = t2370 * t830 * t9716;
    let t9721 = t831 * t2494;
    let t9723 = t2370 * t830 * t9721;
    let t9726 = t3039 * t2358;
    let t9729 = t1114 * t4409;
    (t9709, t9716, t9718, t9721, t9723, t9726, t9729)
}
