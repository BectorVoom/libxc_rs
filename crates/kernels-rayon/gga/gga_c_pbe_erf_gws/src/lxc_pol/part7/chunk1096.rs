//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1096/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1096(t2352: f64, t6781: f64, t829: f64, t830: f64, t4394: f64, t745: f64, t825: f64, t2219: f64, t898: f64, t938: f64, t2365: f64, t4395: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19621 = t6781 * t2352;
    let t19623 = t829 * t830 * t19621;
    let t19626 = t4394 * t745;
    let t19627 = t19626 * t825;
    let t19631 = t2219 * t898;
    let t19632 = t19631 * t938;
    let t19634 = t829 * t830 * t19632;
    let t19637 = t4395 * t2365;
    (t19623, t19626, t19627, t19631, t19634, t19637)
}
