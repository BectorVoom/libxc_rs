//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1205/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1205(t2470: f64, t26270: f64, t7284: f64, t96220: f64, t9675: f64, t94771: f64, t7514: f64, t9288: f64, t7289: f64, t26277: f64, t94776: f64, t25950: f64, t26292: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t96276 = t26270 * t2470;
    let t96277 = t7284 * t96276;
    let t96279 = t96220 * t9675;
    let t96280 = t94771 * t96279;
    let t96282 = t7514 * t9288;
    let t96284 = 0.39982213492741449076e-1_f64 * t7289 * t96282;
    let t96287 = t94776 * t26277;
    let t96289 = t25950 * t26292;
    (t96276, t96277, t96279, t96280, t96282, t96284, t96287, t96289)
}
