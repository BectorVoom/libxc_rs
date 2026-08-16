//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1493/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1493(t17: f64, t19573: f64, t6320: f64, t750: f64, t1388: f64, t1799: f64, t15877: f64, t11979: f64, t15890: f64, t15895: f64, t588: f64, t6328: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19574 = t17 * t19573;
    let t19575 = t6320 * t750;
    let t19576 = t17 * t19575;
    let t19577 = t1799 * t1388;
    let t19581 = 16.0_f64 * t15877;
    let t19588 = 32.0_f64 * t11979;
    let t19589 = 0.34631718211362927517e2_f64 * t15890;
    let t19590 = 0.11696447245269292414e1_f64 * t15895;
    let t19591 = t588 * t6328;
    (t19574, t19576, t19577, t19581, t19588, t19589, t19590, t19591)
}
