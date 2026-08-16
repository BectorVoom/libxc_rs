//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 941/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk941(t9595: f64, t9597: f64, t9600: f64, t9603: f64, t9607: f64, t9610: f64, t9614: f64, t9616: f64, t9618: f64, t9621: f64, t9625: f64, t9628: f64, t9631: f64) -> f64 {
    let t10886 = 0.57970906942607043472e-5_f64 * t9595 - 0.57970906942607043472e-5_f64 * t9597 + 0.86956360413910565208e-5_f64 * t9600 - 0.12380169846338434109e-5_f64 * t9603 + 0.10136107947527008247e-3_f64 * t9607 - 0.34752370105806885418e-3_f64 * t9610 - 0.34752370105806885418e-3_f64 * t9614 - 0.24326659074064819793e-2_f64 * t9616 + 0.84540905957968605064e-6_f64 * t9618 - 0.27801896084645508334e-2_f64 * t9621 + 0.20240885416666666668e-4_f64 * t9625 + 0.10120442708333333334e-3_f64 * t9628 + 0.10120442708333333334e-3_f64 * t9631;
    t10886
}
