//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta558 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1968;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta558(t4147: f64, t7311: f64, t1925: f64, t36: f64, t1353: f64, t2033: f64, t1518: f64, t1931: f64, t7933: f64, t1469: f64, t1450: f64, t11239: f64, t3268: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t32113, t32737, t33602, t33651, t34176, t35669, t36870) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1968(t4147, t7311, t1925, t36, t1353, t2033, t1518, t1931, t7933, t1469, t1450, t11239, t3268);
    (t32113, t32737, t33602, t33651, t34176, t35669, t36870)
}
