//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 788/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk788(t1609: f64, t1986: f64, t7244: f64, t8447: f64, t205: f64, t24985: f64, t3350: f64, t671: f64, t16155: f64, t8516: f64, t8519: f64, t5542: f64, t8687: f64) -> (f64, f64, f64, f64, f64) {
    let t38397 = t1986 * t1609;
    let t38414 = t7244 * t8447;
    let t38454 = t671 * t24985 * t205 * t3350;
    let t38460 = t8516 * t16155 * t8519;
    let t38471 = t8687 * t5542;
    (t38397, t38414, t38454, t38460, t38471)
}
