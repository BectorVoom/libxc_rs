//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1043/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1043(t25560: f64, t8207: f64, t769: f64, t935: f64, t3916: f64, t530: f64, t864: f64, t3881: f64, t287: f64, t320: f64, t321: f64, t3695: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25561 = t8207 * t25560;
    let t25564 = t935 * t769;
    let t25570 = t3916 * t25560;
    let t25622 = t530 * t864;
    let t25742 = t3881 * t25560;
    let t25788 = 0.85858385084333410912e-1_f64 * t320 * t321 * t3695 * t287;
    (t25561, t25564, t25570, t25622, t25742, t25788)
}
