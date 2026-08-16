//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta113 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk670;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk671;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta113(t143: f64, t680: f64, t130: f64, t700: f64, t701: f64, t2435: f64, t2439: f64, t2502: f64, t2504: f64, t2509: f64, t2511: f64, t682: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2564, t2565, t2566, t2567, t2569) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk670(t143, t680, t130, t700, t701);
        let (t2576, t2577, t2579) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk671(t2435, t2439, t2502, t2504, t2509, t2511, t701, t682);
    (t2564, t2565, t2566, t2567, t2569, t2576, t2577, t2579)
}
