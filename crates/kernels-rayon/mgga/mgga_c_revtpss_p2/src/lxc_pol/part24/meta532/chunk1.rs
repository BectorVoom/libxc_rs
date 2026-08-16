//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1570/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1570(t125: f64, t22857: f64, t22809: f64, t22953: f64, t6843: f64, t9994: f64, t6869: f64, t73731: f64, t9816: f64, t9818: f64, t22829: f64, t9962: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t85553 = t125 * t22857;
    let t85563 = t125 * t22809;
    let t85609 = t125 * t22953;
    let t85638 = t6843 * t9994;
    let t85648 = t9816 * t9818 * t73731 * t6869;
    let t85652 = t9962 * t22829;
    (t85553, t85563, t85609, t85638, t85648, t85652)
}
