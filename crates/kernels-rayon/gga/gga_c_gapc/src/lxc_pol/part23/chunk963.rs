//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 963/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk963(t3284: f64, t7200: f64, t11741: f64, t11387: f64, t2580: f64, t7204: f64, t11483: f64, t933: f64, t2597: f64, t7735: f64, t277: f64, t655: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11742 = t3284 * t7200;
    let t11743 = t11741 * t11742;
    let t11745 = t11387 * t2580;
    let t11746 = t7204 * t11745;
    let t11748 = t933 * t11483;
    let t11749 = t2597 * t7735;
    let t11750 = t11748 * t11749;
    let t11752 = t277 * t655;
    (t11742, t11743, t11745, t11746, t11748, t11749, t11750, t11752)
}
