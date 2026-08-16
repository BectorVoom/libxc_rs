//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta607 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1946;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta607(t1583: f64, t4537: f64, t27383: f64, t6079: f64, t775: f64, t890: f64, t98785: f64, t25207: f64, t77408: f64, t18498: f64, t27159: f64, t1468: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t106554, t106555, t106561, t106562, t106565, t106566, t106569, t106572, t106583) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1946(t1583, t4537, t27383, t6079, t775, t890, t98785, t25207, t77408, t18498, t27159, t1468);
    (t106554, t106555, t106561, t106562, t106565, t106566, t106569, t106572, t106583)
}
