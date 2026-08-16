//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 869/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk869(t3519: f64, t663: f64, t685: f64, t1084: f64, t7489: f64, t5522: f64, t5745: f64, t7357: f64, t7500: f64, t9148: f64, t9163: f64, t228: f64) -> (f64, f64, f64, f64, f64) {
    let t9334 = t3519 * t663;
    let t9336 = 1.0_f64 * t9334 * t685;
    let t9338 = 2.0_f64 * t7489 * t1084;
    let t9343 = -t5745 + 0.23744444444444444444e-1_f64 * t5522 + 0.47488888888888888888e-1_f64 * t7357 - t7500 - 0.17808333333333333333e-1_f64 * t9148 + 0.53425e-1_f64 * t9163;
    let t9345 = 0.621814e-1_f64 * t9343 * t228;
    (t9334, t9336, t9338, t9343, t9345)
}
