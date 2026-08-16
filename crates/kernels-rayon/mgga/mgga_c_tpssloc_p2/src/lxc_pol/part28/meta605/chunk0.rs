//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1911/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1911(t1985: f64, t7700: f64, t80707: f64, t214: f64, t5318: f64, t6888: f64, t6891: f64, t16065: f64, t1992: f64, t22897: f64, t26378: f64, t6914: f64) -> (f64, f64, f64, f64, f64) {
    let t90737 = t1985 * t80707 * t7700;
    let t90739 = t214 * t5318;
    let t90741 = t6888 * t90739 * t6891;
    let t90747 = t1992 * t22897 * t16065;
    let t90749 = t6914 * t26378;
    (t90737, t90739, t90741, t90747, t90749)
}
