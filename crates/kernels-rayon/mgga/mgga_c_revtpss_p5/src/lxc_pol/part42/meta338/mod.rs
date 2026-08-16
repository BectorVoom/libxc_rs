//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta338 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1141;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta338(t15191: f64, t4628: f64, t698: f64, t15127: f64, t15125: f64, t3014: f64, t4707: f64, t15168: f64, t4682: f64, t964: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15192, t15197, t15198, t15209, t15210, t15211, t15258, t15301, t15312, t15322, t15324, t15343) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1141(t15191, t4628, t698, t15127, t15125, t3014, t4707, t15168, t4682, t964);
    (t15192, t15197, t15198, t15209, t15210, t15211, t15258, t15301, t15312, t15322, t15324, t15343)
}
