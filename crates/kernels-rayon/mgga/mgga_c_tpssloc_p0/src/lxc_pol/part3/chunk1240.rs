//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1240/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1240(t12189: f64, t1811: f64, t1358: f64, t5231: f64, t16123: f64, t554: f64, t1815: f64, t3862: f64, t3726: f64, t5227: f64, t119: f64, t16018: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16341 = t12189 * t1811;
    let t16346 = 7.0_f64 / 2304.0_f64 * t5231 * t1358;
    let t16347 = t16123 * t554;
    let t16350 = t1815 * t3862;
    let t16354 = 7.0_f64 / 72.0_f64 * t3726 * t5227;
    let t16355 = t119 * t16018;
    (t16341, t16346, t16347, t16350, t16354, t16355)
}
