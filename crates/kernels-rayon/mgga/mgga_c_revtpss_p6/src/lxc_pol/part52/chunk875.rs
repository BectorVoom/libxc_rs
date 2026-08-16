//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 875/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk875(t25900: f64, t26230: f64, t25904: f64, t3916: f64, t25895: f64, t3920: f64, t7496: f64, t2098: f64, t2453: f64, t3908: f64, t7507: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26231 = t26230 * t25900;
    let t26232 = t25904 * t26231;
    let t26234 = t26230 * t3916;
    let t26235 = t25895 * t26234;
    let t26238 = 0.13009920719177044025e-1_f64 * t7496 * t3920;
    let t26249 = t2453 * t2098;
    let t26251 = 0.11565819519348392139e-2_f64 * t26249 * t3908;
    let t26252 = t786 * t7507;
    (t26231, t26232, t26234, t26235, t26238, t26251, t26252)
}
