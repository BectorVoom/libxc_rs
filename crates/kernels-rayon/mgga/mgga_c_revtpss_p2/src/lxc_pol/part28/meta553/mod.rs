//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta553 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2004;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta553(t2435: f64, t25352: f64, t11015: f64, t7018: f64, t7048: f64, t822: f64, t25300: f64, t9285: f64, t25299: f64, t7059: f64, t9288: f64, t7064: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t92858, t92861, t92864, t92868, t92870, t92871, t92873) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2004(t2435, t25352, t11015, t7018, t7048, t822, t25300, t9285, t25299, t7059, t9288, t7064);
    (t92858, t92861, t92864, t92868, t92870, t92871, t92873)
}
