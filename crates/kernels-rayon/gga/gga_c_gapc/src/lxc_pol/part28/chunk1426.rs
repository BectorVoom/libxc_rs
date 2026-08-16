//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1426/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1426(t12047: f64, t12061: f64, t12046: f64, t12059: f64, t12039: f64, t12042: f64, t2845: f64, t36095: f64, t36098: f64, t36100: f64, t36103: f64, t36105: f64, t36108: f64, t36109: f64, t36111: f64, t36113: f64, t36116: f64, t36119: f64, t36122: f64, t36124: f64, t36270: f64, t36271: f64, t36275: f64, t36283: f64, t36285: f64, t37312: f64, t3848: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37325 = 4.0_f64 * t12047;
    let t37326 = 2.0_f64 * t12061;
    let t37327 = 2.0_f64 * t12046;
    let t37328 = 4.0_f64 * t12059;
    let t37329 = 2.0_f64 * t12039;
    let t37330 = 2.0_f64 * t12042;
    let t38852 = t2845 * t3848 - t36095 - t36098 - t36100 - t36103 + t36105 - t36108 - t36109 + t36111 - t36113 - t36116 + t36119 + t36122 + t36124 - t36270 - t36271 - t36275 + t36283 - t36285 + t37312;
    (t37325, t37326, t37327, t37328, t37329, t37330, t38852)
}
