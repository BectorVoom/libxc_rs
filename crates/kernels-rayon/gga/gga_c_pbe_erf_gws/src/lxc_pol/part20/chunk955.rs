//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 955/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk955(t3440: f64, t401: f64, t3434: f64, t3437: f64, t572: f64, t9788: f64, t606: f64, t10438: f64, t10443: f64, t1856: f64, t3342: f64, t4957: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10756 = t401 * t3440;
    let t10758 = t401 * t3434;
    let t10760 = t401 * t3437;
    let t10762 = t572 * t9788;
    let t10763 = t606 * t10762;
    let t10771 = t606 * t10438;
    let t10774 = t1856 * t10443;
    let t10777 = t4957 * t3342;
    (t10756, t10758, t10760, t10762, t10763, t10771, t10774, t10777)
}
