//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta436 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1643;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1644;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta436(t3498: f64, t5205: f64, t1196: f64, t12485: f64, t1756: f64, t3524: f64, t3531: f64, t5198: f64, t12361: f64, t5068: f64, t12243: f64, t5109: f64, t1149: f64, t5105: f64, t3384: f64, t1733: f64, t3427: f64, t3385: f64, t5108: f64, t12248: f64, t3435: f64, t5104: f64, t3433: f64, t12230: f64, t1732: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16641, t16645, t16647, t16649, t16651) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1643(t3498, t5205, t1196, t12485, t1756, t3524, t3531, t5198, t12361, t5068, t12243, t5109);
        let (t16654, t16657, t16660, t16664, t16667, t16668) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1644(t1149, t5105, t3384, t1733, t3427, t3385, t5108, t12248, t3435, t5104, t3433, t12230, t1732);
    (t16641, t16645, t16647, t16649, t16651, t16654, t16657, t16660, t16664, t16667, t16668)
}
