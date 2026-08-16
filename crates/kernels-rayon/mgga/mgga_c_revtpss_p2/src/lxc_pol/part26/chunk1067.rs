//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1067/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1067(t25900: f64, t26230: f64, t25904: f64, t3916: f64, t25895: f64, t3920: f64, t7496: f64, t1398: f64, t543: f64, t7506: f64, t7301: f64, t2097: f64, t4056: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26231 = t26230 * t25900;
    let t26232 = t25904 * t26231;
    let t26234 = t26230 * t3916;
    let t26235 = t25895 * t26234;
    let t26238 = 0.13009920719177044025e-1_f64 * t7496 * t3920;
    let t26240 = t7506 * t1398 * t543;
    let t26241 = t7301 * t26240;
    let t26246 = t7301 * t2097 * t4056 * t543;
    (t26231, t26232, t26234, t26235, t26238, t26241, t26246)
}
