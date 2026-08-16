//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1102/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1102(t2033: f64, t549: f64, t9943: f64, t23203: f64, t959: f64, t2012: f64, t7802: f64, t9797: f64, t2586: f64, t2679: f64, t9796: f64, t2013: f64, t9813: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28729 = 0.11916829983950142223e0_f64 * t2033 * t549 * t9943;
    let t28731 = 0.29792074959875355558e-1_f64 * t23203 * t959;
    let t28737 = t2012 * t7802;
    let t28738 = t28737 * t9797;
    let t28742 = t9796 * t2586 * t2679;
    let t28792 = t2013 * t9813;
    (t28729, t28731, t28737, t28738, t28742, t28792)
}
