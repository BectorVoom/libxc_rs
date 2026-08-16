//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1191/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1191(t119968: f64, t126375: f64, t119836: f64, t31854: f64, t33711: f64, t120082: f64, t33716: f64, t119935: f64, t33674: f64, t31834: f64, t33722: f64, t14691: f64, t246: f64, t31851: f64, t8486: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t126376 = t119968 * t126375;
    let t126378 = t119836 * t126375;
    let t126384 = t33711 * t31854;
    let t126386 = t120082 * t33716;
    let t126388 = t119935 * t33674;
    let t126390 = t31834 * t33722;
    let t126394 = t8486 * t31851 * t246 * t14691;
    (t126376, t126378, t126384, t126386, t126388, t126390, t126394)
}
