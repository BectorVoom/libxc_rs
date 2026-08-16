//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta590 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2005;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta590(t25310: f64, t25331: f64, t2435: f64, t25339: f64, t11064: f64, t7086: f64, t1113: f64, t2411: f64, t530: f64, t7311: f64, t2470: f64, t26049: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t93384, t93391, t93404, t94245, t94345, t94377) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2005(t25310, t25331, t2435, t25339, t11064, t7086, t1113, t2411, t530, t7311, t2470, t26049);
    (t93384, t93391, t93404, t94245, t94345, t94377)
}
