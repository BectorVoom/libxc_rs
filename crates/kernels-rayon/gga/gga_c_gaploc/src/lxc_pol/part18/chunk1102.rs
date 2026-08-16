//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1102/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1102(t28683: f64, t5840: f64, t9890: f64, t2017: f64, t3295: f64, t825: f64, t2033: f64, t549: f64, t9943: f64, t23203: f64, t959: f64, t2012: f64, t7802: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28684 = 0.5396411800922179584e0_f64 * t28683;
    let t28714 = t5840 * t9890;
    let t28715 = 0.1022478025437886658e1_f64 * t28714;
    let t28726 = 0.11928910296775344344e1_f64 * t825 * t2017 * t3295;
    let t28729 = 0.11916829983950142223e0_f64 * t2033 * t549 * t9943;
    let t28731 = 0.29792074959875355558e-1_f64 * t23203 * t959;
    let t28737 = t2012 * t7802;
    (t28684, t28715, t28726, t28729, t28731, t28737)
}
