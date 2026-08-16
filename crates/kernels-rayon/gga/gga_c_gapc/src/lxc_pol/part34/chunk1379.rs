//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1379/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1379(t33704: f64, t33707: f64, t33710: f64, t33714: f64, t33717: f64, t33726: f64, t33728: f64, t33731: f64, t33734: f64, t33719: f64, t36671: f64, t33741: f64) -> (f64, f64) {
    let t36672 = 0.43440462632258606772e-4_f64 * t33704;
    let t36673 = 0.21720231316129303386e-4_f64 * t33707;
    let t36674 = 0.41223756048076119805e-5_f64 * t33710;
    let t36675 = 0.73295838253479341016e-5_f64 * t33714;
    let t36676 = 0.73744819641113281254e-8_f64 * t33717;
    let t36678 = 0.40481770833333333336e-4_f64 * t33726;
    let t36679 = 0.11372686522837130914e-5_f64 * t33728;
    let t36680 = 0.11372686522837130914e-5_f64 * t33731;
    let t36681 = 0.4637672555408563478e-4_f64 * t33734;
    let t36682 = -t36671 - t36672 - t36673 - t36674 + t36675 - t36676 + 0.12650553385416666668e-5_f64 * t33719 + t36678 - t36679 - t36680 + t36681;
    let t36687 = 0.43284943850479925795e-3_f64 * t33741;
    (t36682, t36687)
}
