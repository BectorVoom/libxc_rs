//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1028/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1028(t12189: f64, t1811: f64, t1358: f64, t5231: f64, t1815: f64, t3862: f64, t3726: f64, t5227: f64, t3802: f64, t5234: f64, t3788: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16341 = t12189 * t1811;
    let t16346 = 7.0_f64 / 2304.0_f64 * t5231 * t1358;
    let t16350 = t1815 * t3862;
    let t16354 = 7.0_f64 / 72.0_f64 * t3726 * t5227;
    let t16394 = t5234 * t3802;
    let t16397 = t3788 * t836;
    (t16341, t16346, t16350, t16354, t16394, t16397)
}
