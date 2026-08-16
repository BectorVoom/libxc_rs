//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 871/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk871(t730: f64, t9359: f64, t2746: f64, t2783: f64, t3525: f64, t5734: f64, t1850: f64, t3551: f64, t5522: f64, t5783: f64, t7357: f64, t7420: f64, t9138: f64, t9140: f64, t9143: f64, t9148: f64, t9163: f64, t9165: f64, t9172: f64, t9174: f64) -> (f64, f64, f64, f64, f64) {
    let t9361 = 0.34631718211362927518e2_f64 * t730 * t9359;
    let t9363 = 2.0_f64 * t2746 * t2783;
    let t9365 = 2.0_f64 * t5734 * t3525;
    let t9367 = 1.0_f64 * t1850 * t3551;
    let t9378 = 0.142419375e1_f64 * t9138 - 0.1898925e1_f64 * t9140 - 0.9494625e0_f64 * t9143 + 0.1898925e1_f64 * t9165 - t5783 + 0.39862222222222222223e0_f64 * t5522 + 0.79724444444444444445e0_f64 * t7357 - t7420 - 0.29896666666666666667e0_f64 * t9148 + 0.8969e0_f64 * t9163 - 0.76790625e-1_f64 * t9172 + 0.3071625e0_f64 * t9174;
    (t9361, t9363, t9365, t9367, t9378)
}
