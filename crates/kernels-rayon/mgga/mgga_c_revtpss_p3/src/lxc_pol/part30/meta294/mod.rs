//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta294 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1277;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta294(t555: f64, t9646: f64, t1358: f64, t22: f64, t1425: f64, t225: f64, t3907: f64, t9285: f64, t3906: f64, t1357: f64, t4132: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9648, t9650, t9655, t9656, t9657, t9664, t9666, t9668) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1277(t555, t9646, t1358, t22, t1425, t225, t3907, t9285, t3906, t1357, t4132, t689);
    (t9648, t9650, t9655, t9656, t9657, t9664, t9666, t9668)
}
