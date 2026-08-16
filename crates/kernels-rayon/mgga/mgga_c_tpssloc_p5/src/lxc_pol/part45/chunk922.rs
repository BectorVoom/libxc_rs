//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 922/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk922(t30671: f64, t6547: f64, t23030: f64, t30660: f64, t23204: f64, t30656: f64, t6562: f64, t30624: f64, t81591: f64, t23270: f64, t2379: f64, t25038: f64, t30622: f64) -> (f64, f64, f64, f64, f64) {
    let t112673 = t6547 * t30671;
    let t112674 = 0.76763589786250567036e-1_f64 * t112673;
    let t112676 = 0.52089578783527170489e-1_f64 * t23030 * t30660;
    let t112678 = t6562 * t23204 * t30656;
    let t112679 = 0.16449340668482264365e-1_f64 * t112678;
    let t112680 = t81591 * t30624;
    let t112681 = 0.15352717957250113407e0_f64 * t112680;
    let t112685 = 0.9869604401089358619e-1_f64 * t25038 * t23270 * t30622 * t2379;
    (t112674, t112676, t112679, t112681, t112685)
}
