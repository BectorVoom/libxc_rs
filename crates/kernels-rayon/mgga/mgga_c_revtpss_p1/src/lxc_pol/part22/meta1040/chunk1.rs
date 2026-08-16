//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3633/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3633(t20644: f64, t3427: f64, t3433: f64, t68738: f64, t68742: f64, t68744: f64, t68746: f64, t68748: f64, t68751: f64, t68754: f64, t68757: f64, t68760: f64, t68763: f64, t68766: f64, t68769: f64) -> (f64, f64) {
    let t68772 = 0.16081979498692535067e2_f64 * t3433 * t20644 * t3427;
    let t68773 = t68738 + t68742 + t68744 - t68746 - t68748 + t68751 + t68754 - t68757 - t68760 - t68763 - t68766 - t68769 + t68772;
    (t68772, t68773)
}
