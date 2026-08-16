//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1140/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1140(t120082: f64, t33716: f64, t119935: f64, t33674: f64, t31834: f64, t33722: f64, t14691: f64, t246: f64, t31851: f64, t8486: f64, t120042: f64, t1549: f64) -> (f64, f64, f64, f64, f64) {
    let t126386 = t120082 * t33716;
    let t126388 = t119935 * t33674;
    let t126390 = t31834 * t33722;
    let t126394 = t8486 * t31851 * t246 * t14691;
    let t126396 = t120042 * t1549;
    (t126386, t126388, t126390, t126394, t126396)
}
