//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1055/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1055(t321: f64, t3695: f64, t429: f64, t457: f64, t27059: f64, t466: f64, t115: f64, t25834: f64, t426: f64, t3209: f64, t1724: f64, t9166: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27651 = 0.85858385084333410912e-1_f64 * t457 * t321 * t3695 * t429;
    let t27670 = 0.5224665647534064904e-2_f64 * t466 * t27059;
    let t27705 = t426 * t25834 * t115;
    let t27706 = t3209 * t27705;
    let t27712 = t1724 * t27705;
    let t27780 = t9166 * sigma2;
    (t27651, t27670, t27705, t27706, t27712, t27780)
}
