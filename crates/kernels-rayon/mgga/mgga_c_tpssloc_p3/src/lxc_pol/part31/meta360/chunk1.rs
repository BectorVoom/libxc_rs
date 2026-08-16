//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1279/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1279(t16094: f64, t16097: f64, t12214: f64, t131: f64, t205: f64, t3726: f64, t5206: f64, t12199: f64, t5202: f64, t118: f64, t5187: f64, t794: f64) -> (f64, f64, f64, f64, f64) {
    let t16099 = 0.49999999999999999998e-2_f64 * t16094 * t16097;
    let t16100 = t12214 * t131;
    let t16101 = t205 * t16100;
    let t16106 = t3726 * t5206;
    let t16108 = t12199 * t5202;
    let t16111 = t118 * t794 * t5187;
    (t16099, t16101, t16106, t16108, t16111)
}
