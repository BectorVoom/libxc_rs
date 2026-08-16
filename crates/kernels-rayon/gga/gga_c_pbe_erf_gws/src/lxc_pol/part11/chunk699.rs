//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 699/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk699(t825: f64, t9847: f64, t1114: f64, t3047: f64, t3083: f64, t3052: f64, t3724: f64, t840: f64, t1161: f64, t8589: f64, t829: f64, t830: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9848 = t9847 * t825;
    let t9849 = t1114 * t9848;
    let t9852 = t3083 * t3047;
    let t9854 = t3083 * t3052;
    let t9879 = t840 * t3724;
    let t9883 = t8589 * t1161;
    let t9885 = t829 * t830 * t9883;
    (t9848, t9849, t9852, t9854, t9879, t9885)
}
