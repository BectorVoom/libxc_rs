//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 589/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk589(t2395: f64, t814: f64, t829: f64, t830: f64, t2100: f64, t831: f64, t2228: f64, t840: f64, t2367: f64, t2373: f64, t2306: f64, t2365: f64) -> (f64, f64, f64, f64, f64) {
    let t4459 = t829 * t830 * t2395 * t814;
    let t4464 = t829 * t830 * t831 * t2100;
    let t4467 = t840 * t2228;
    let t4469 = t2367 * t2373;
    let t4473 = t2306 * t2365;
    (t4459, t4464, t4467, t4469, t4473)
}
