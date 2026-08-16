//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta229 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1070;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta229(t300: f64, t5188: f64, t5156: f64, t1749: f64, t1198: f64, t1765: f64, t3531: f64, t1756: f64, t3495: f64, t1189: f64, t1196: f64, t1179: f64, t1188: f64, t5180: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5189, t5191, t5192, t5194, t5196, t5197, t5198, t5200, t5202) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1070(t300, t5188, t5156, t1749, t1198, t1765, t3531, t1756, t3495, t1189, t1196, t1179, t1188, t5180);
    (t5189, t5191, t5192, t5194, t5196, t5197, t5198, t5200, t5202)
}
