//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 913/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk913(t17174: f64, t16746: f64, t587: f64, t590: f64, t591: f64, t1663: f64, t187: f64, t22: f64, t16740: f64, t197: f64, t16669: f64, t2620: f64, t592: f64) -> (f64, f64, f64, f64) {
    let t17175 = 256.0_f64 / 243.0_f64 * t17174;
    let t17179 = 4.0_f64 / 45.0_f64 * t587 * t590 * t591 * t16746;
    let t17182 = t22 / t187 / t1663;
    let t17183 = t197 * t16740;
    let t17187 = 352.0_f64 / 243.0_f64 * t587 * t17182 * t17183 * t16669;
    let t17188 = t2620 * t592;
    (t17175, t17179, t17187, t17188)
}
