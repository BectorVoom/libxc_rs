//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1065/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1065(t13518: f64, t2142: f64, t13609: f64, t840: f64, t13628: f64, t1161: f64, t35889: f64, t829: f64, t830: f64, t13096: f64, t34857: f64, t1105: f64, t12232: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46615 = t13518 * t2142;
    let t46635 = t840 * t13609;
    let t46637 = t840 * t13628;
    let t46639 = t35889 * t1161;
    let t46641 = t829 * t830 * t46639;
    let t46650 = t34857 * t13096;
    let t46654 = t12232 * t1105;
    (t46615, t46635, t46637, t46641, t46650, t46654)
}
