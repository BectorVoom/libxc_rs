//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta558 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1877;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta558(t1873: f64, t94519: f64, t94520: f64, t94527: f64, t94537: f64, t94540: f64, t26004: f64, t5690: f64, t13951: f64, t2018: f64, t807: f64, t94565: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98260, t98263, t98264, t98267, t98268, t98269, t98281, t98283) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1877(t1873, t94519, t94520, t94527, t94537, t94540, t26004, t5690, t13951, t2018, t807, t94565);
    (t98260, t98263, t98264, t98267, t98268, t98269, t98281, t98283)
}
