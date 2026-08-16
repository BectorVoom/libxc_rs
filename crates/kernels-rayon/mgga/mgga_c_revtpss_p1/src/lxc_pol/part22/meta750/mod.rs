//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta750 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2823;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta750(t11408: f64, t941: f64, t2979: f64, t2986: f64, t11465: f64, t960: f64, t2935: f64, t2967: f64, t11509: f64, t3006: f64, t2866: f64, t2873: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t41779, t41785, t41788, t41799, t41813, t41880) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2823(t11408, t941, t2979, t2986, t11465, t960, t2935, t2967, t11509, t3006, t2866, t2873);
    (t41779, t41785, t41788, t41799, t41813, t41880)
}
