//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta232 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk990;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta232(t13126: f64, t460: f64, t12051: f64, t471: f64, t11239: f64, t3596: f64, t3603: f64, t13038: f64, t13045: f64, t1275: f64, t225: f64, t1466: f64, t2246: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13127, t13129, t13141, t13142, t13143, t13147, t13148, t13149, t13180, t13182, t13272) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk990(t13126, t460, t12051, t471, t11239, t3596, t3603, t13038, t13045, t1275, t225, t1466, t2246);
    (t13127, t13129, t13141, t13142, t13143, t13147, t13148, t13149, t13180, t13182, t13272)
}
