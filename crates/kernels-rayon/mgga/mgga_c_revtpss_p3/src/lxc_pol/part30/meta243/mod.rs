//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta243 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1084;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1085;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1086;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta243(t1294: f64, t1828: f64, t3737: f64, t1284: f64, t1770: f64, t1280: f64, t5230: f64, t1287: f64, t5346: f64, t1774: f64, t3759: f64, t5245: f64, t354: f64, t471: f64, t1214: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5428, t5429) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1084(t1294, t1828, t3737);
        let (t5436, t5443, t5446, t5449, t5452, t5457) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1085(t1284, t1770, t1280, t5230, t1287, t5346, t1774, t3759, t5245, t354, t471);
        let t5458 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1086(t1214, t5457);
    (t5428, t5429, t5436, t5443, t5446, t5449, t5452, t5457, t5458)
}
