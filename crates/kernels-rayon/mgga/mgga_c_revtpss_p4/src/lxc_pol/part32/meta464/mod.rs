//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta464 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1690;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta464(t26179: f64, t6960: f64, t2047: f64, t25163: f64, t6963: f64, t7349: f64, t10301: f64, t7342: f64, t6954: f64, t239: f64, t72: f64, t1927: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t26180, t26182, t26185, t26187, t26190, t26204, t26205) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1690(t26179, t6960, t2047, t25163, t6963, t7349, t10301, t7342, t6954, t239, t72, t1927);
    (t26180, t26182, t26185, t26187, t26190, t26204, t26205)
}
