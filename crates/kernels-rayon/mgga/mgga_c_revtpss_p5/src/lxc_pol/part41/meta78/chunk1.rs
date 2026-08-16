//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 466/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk466(t1600: f64, t916: f64, t923: f64, t1592: f64, t930: f64, t141: f64, t1594: f64, t921: f64, t929: f64, t935: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1601 = t916 * t1600;
    let t1604 = t923 * t1600;
    let t1606 = t930 * t1592;
    let t1607 = t141 * t1606;
    let t1609 = 0.1898925e1_f64 * t1601 - t921 - 0.29896666666666666667e0_f64 * t1594 + 0.3071625e0_f64 * t1604 - t929 - 0.82156666666666666667e-1_f64 * t1607;
    let t1610 = t1609 * t935;
    (t1601, t1604, t1606, t1607, t1609, t1610)
}
