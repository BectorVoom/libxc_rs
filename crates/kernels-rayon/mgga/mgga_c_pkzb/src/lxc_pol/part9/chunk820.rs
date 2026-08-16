//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 820/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk820(t237: f64, t5880: f64, t5909: f64, t5488: f64, t5504: f64, t5580: f64, t5765: f64, t5768: f64, t5770: f64, t5773: f64, t5779: f64, t5799: f64, t5807: f64, t5811: f64) -> (f64, f64) {
    let t5911 = t237 * (t5880 + t5909);
    let t5912 = -t5580 + t5765 + t5768 + t5770 + t5773 - t5779 + t5799 + t5807 - t5488 + t5811 + t5504 + t5911;
    (t5911, t5912)
}
