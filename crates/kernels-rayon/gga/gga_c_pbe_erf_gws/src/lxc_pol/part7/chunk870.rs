//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 870/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk870(t1663: f64, t16679: f64, t11: f64, t1758: f64, t4360: f64, t4962: f64, t16669: f64, t4957: f64, t571: f64, t16613: f64, t1764: f64, t4971: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16680 = t1663 * t16679;
    let t16682 = t11 * t1758 * t16680;
    let t16684 = t4962 * t4360;
    let t16686 = t11 * t1758 * t16684;
    let t16688 = t4957 * t16669;
    let t16690 = t11 * t571 * t16688;
    let t16693 = t11 * t571 * t16613;
    let t16695 = t1764 * t16679;
    let t16697 = t11 * t571 * t16695;
    let t16699 = t4971 * t4360;
    (t16680, t16682, t16684, t16686, t16688, t16690, t16693, t16695, t16697, t16699)
}
