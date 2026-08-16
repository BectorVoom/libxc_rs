//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta351 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1365;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta351(t14005: f64, t9816: f64, t2713: f64, t3964: f64, t5617: f64, t5686: f64, t9744: f64, t221: f64, t4019: f64, t5659: f64, t4018: f64, t3989: f64, t5629: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t14007, t14013, t14024, t14036, t14038, t14040) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1365(t14005, t9816, t2713, t3964, t5617, t5686, t9744, t221, t4019, t5659, t4018, t3989, t5629);
    (t14007, t14013, t14024, t14036, t14038, t14040)
}
