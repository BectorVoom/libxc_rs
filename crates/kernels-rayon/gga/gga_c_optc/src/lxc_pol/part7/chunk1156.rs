//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1156/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1156(t2472: f64, t3802: f64, t7604: f64, t845: f64, t2367: f64, t7300: f64, t999: f64, t2441: f64, t7344: f64, t2435: f64, t2436: f64, t7266: f64) -> (f64, f64, f64, f64) {
    let t24037 = 0.69263023597503453196e2_f64 * t845 * t2472 * t7604 * t3802;
    let t24041 = t999 * t2367 * t7300;
    let t24044 = 0.4155781415850207192e3_f64 * t2441 * t7344;
    let t24046 = t2435 * t2436 * t7266;
    (t24037, t24041, t24044, t24046)
}
