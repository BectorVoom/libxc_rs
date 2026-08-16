//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta343 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1355;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta343(t1222: f64, t13011: f64, t3367: f64, t404: f64, t1204: f64, t3140: f64, t3599: f64, t1242: f64, t3603: f64, t471: f64, t3609: f64, t414: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13012, t13026, t13032, t13033, t13038, t13045, t13058, t13099) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1355(t1222, t13011, t3367, t404, t1204, t3140, t3599, t1242, t3603, t471, t3609, t414);
    (t13012, t13026, t13032, t13033, t13038, t13045, t13058, t13099)
}
