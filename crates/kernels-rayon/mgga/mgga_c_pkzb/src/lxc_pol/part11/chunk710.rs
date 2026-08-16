//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 710/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk710(t5089: f64, t555: f64, t12: f64, t137: f64, t139: f64, t24: f64, t1626: f64, t501: f64, t572: f64, t81: f64, t79: f64, t127: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5091 = 0.10389515463408878255e3_f64 * t555 * t5089;
    let t5093 = 1.0_f64 / t137 / t12;
    let t5106 = 1.0_f64 / t139 / t24;
    let t5130 = 12.0_f64 * t501 * t1626;
    let t5135 = t81 * t572;
    let t5136 = 1.0_f64 / t5135;
    let t5137 = t79 * t5136;
    let t5139 = 120.0_f64 * t5137 * t127;
    (t5091, t5093, t5106, t5130, t5135, t5136, t5137, t5139)
}
