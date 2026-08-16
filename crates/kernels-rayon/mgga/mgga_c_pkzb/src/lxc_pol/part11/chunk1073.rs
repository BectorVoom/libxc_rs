//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1073/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1073(t1835: f64, t87: f64, t5829: f64, t690: f64, t1731: f64, t218: f64, t220: f64, t5555: f64, t679: f64, t16194: f64, t213: f64, t778: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17359 = t1835 * t1835;
    let t17361 = 1.0_f64 / t87 / t17359;
    let t17391 = t690 * t5829;
    let t17402 = t218 * t1731 * t220;
    let t17403 = 0.13490888888888888889e1_f64 * t17402;
    let t17405 = t218 * t5555 * t679;
    let t17432 = 1.0_f64 / t213 / t16194 / t778 / 96.0_f64;
    (t17361, t17391, t17402, t17403, t17405, t17432)
}
