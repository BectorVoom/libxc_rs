//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta122 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk607;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk608;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk609;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta122(t1032: f64, t989: f64, t1040: f64, t1024: f64, t1062: f64, t1031: f64, t196: f64, t342: f64, t1034: f64, t358: f64, t360: f64, t368: f64, t335: f64, t365: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3124, t3127) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk607(t1032, t989, t1040, t1024, t1062);
        let t3140 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk608(t1031, t196);
        let (t3141, t3143, t3144, t3145, t3147, t3148, t3149, t3150, t3153) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk609(t3140, t342, t1034, t358, t360, t368, t335, t365, t73);
    (t3124, t3127, t3140, t3141, t3143, t3144, t3145, t3147, t3148, t3149, t3150, t3153)
}
