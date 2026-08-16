//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 962/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk962(t23471: f64, t6740: f64, t1015: f64, t6746: f64, t40: f64, t984: f64, t1933: f64, t225: f64, t343: f64, t364: f64, t6721: f64, t6739: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23472 = t6740 * t23471;
    let t23473 = t1015 * t6746;
    let t23474 = t23472 * t23473;
    let t23476 = t40 * t984;
    let t23477 = t1933 * t23476;
    let t23478 = t343 * t225;
    let t23479 = t23478 * t364;
    let t23480 = t23477 * t23479;
    let t23482 = t6721 * t6739;
    (t23472, t23474, t23476, t23478, t23479, t23480, t23482)
}
