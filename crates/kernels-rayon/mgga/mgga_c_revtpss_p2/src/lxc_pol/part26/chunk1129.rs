//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1129/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1129(t1954: f64, t39643: f64, t7056: f64, t2453: f64, t25309: f64, t25304: f64, t251: f64, t25410: f64, t2438: f64, t837: f64, t786: f64, t92889: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93139 = t1954 * t39643;
    let t93140 = t93139 * t7056;
    let t93157 = t2453 * t25309;
    let t93160 = t25304 * t25309;
    let t93169 = t2453 * t251;
    let t93170 = t93169 * t25410;
    let t93173 = t2438 * t837;
    let t93179 = t786 * t92889;
    (t93139, t93140, t93157, t93160, t93169, t93170, t93173, t93179)
}
