//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1271/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1271(t19815: f64, t6951: f64, t22783: f64, t6427: f64, t236: f64, t6387: f64, t22705: f64, t22852: f64, t550: f64, t28100: f64, t80853: f64, t80855: f64) -> (f64, f64, f64, f64, f64) {
    let t97265 = t19815 * t6951;
    let t97283 = t22783 * t6427;
    let t97312 = t236 * t6387;
    let t97315 = t22852 * t22705 * t97312 * t550;
    let t97347 = t80853 * t80855 * t28100;
    (t97265, t97283, t97312, t97315, t97347)
}
