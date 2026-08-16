//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1704/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1704(t11804: f64, t247: f64, t3116: f64, t11173: f64, t373: f64, t371: f64, t372: f64, t3211: f64, t3215: f64, t1026: f64, t676: f64, t1025: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11806 = t247 * t3116 * t11804;
    let t11809 = t373 * t11173;
    let t11811 = t371 * t372 * t11809;
    let t11814 = t3211 * t3215;
    let t11817 = t371 * t676 * t1026;
    let t11818 = t1025 * t11817;
    (t11806, t11809, t11811, t11814, t11817, t11818)
}
