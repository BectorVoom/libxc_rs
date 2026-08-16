//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 818/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk818(t745: f64, t814: f64, t2129: f64, t2142: f64, t2123: f64, t6183: f64, t2120: f64, t326: f64, t6469: f64, t2200: f64, t855: f64, t859: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6598 = t745 * t814;
    let t6603 = t2129 * t2142;
    let t6605 = t6183 * t2123;
    let t6606 = t2120 * t6605;
    let t6608 = t326 * t6469;
    let t6616 = t855 * t2200 * t859;
    (t6598, t6603, t6605, t6606, t6608, t6616)
}
