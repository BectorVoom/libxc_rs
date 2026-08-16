//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1388/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1388(t25423: f64, t3102: f64, t321: f64, t3695: f64, t429: f64, t457: f64, t3234: f64, t9189: f64, t9197: f64, t27059: f64, t466: f64, t27175: f64, t935: f64) -> (f64, f64, f64, f64, f64) {
    let t27644 = t3102 * t25423;
    let t27651 = 0.85858385084333410912e-1_f64 * t457 * t321 * t3695 * t429;
    let t27667 = t3234 * t9189 * t9197;
    let t27670 = 0.5224665647534064904e-2_f64 * t466 * t27059;
    let t27671 = t27175 * t935;
    (t27644, t27651, t27667, t27670, t27671)
}
