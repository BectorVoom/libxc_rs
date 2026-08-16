//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 339/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk339(t1592: f64, t904: f64, t128: f64, t903: f64, t291: f64, t902: f64, t916: f64, t923: f64, t930: f64, t141: f64, t921: f64, t929: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1593 = t904 * t1592;
    let t1594 = t128 * t1593;
    let t1596 = -t903 - 0.17808333333333333333e-1_f64 * t1594;
    let t1598 = 0.621814e-1_f64 * t1596 * t291;
    let t1600 = -t902 / 3.0_f64 - t1594 / 3.0_f64;
    let t1601 = t916 * t1600;
    let t1604 = t923 * t1600;
    let t1606 = t930 * t1592;
    let t1607 = t141 * t1606;
    let t1609 = 0.1898925e1_f64 * t1601 - t921 - 0.29896666666666666667e0_f64 * t1594 + 0.3071625e0_f64 * t1604 - t929 - 0.82156666666666666667e-1_f64 * t1607;
    (t1593, t1594, t1596, t1598, t1600, t1601, t1604, t1606, t1607, t1609)
}
