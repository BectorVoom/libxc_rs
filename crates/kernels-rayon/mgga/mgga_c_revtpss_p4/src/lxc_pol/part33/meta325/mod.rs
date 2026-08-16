//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta325 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1332;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta325(t2986: f64, t960: f64, t11132: f64, t1034: f64, t3154: f64, t357: f64, t1024: f64, t3105: f64, t905: f64, t606: f64, t1052: f64, t360: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11554, t11560, t11574, t11627, t11631, t11656, t11661, t11670) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1332(t2986, t960, t11132, t1034, t3154, t357, t1024, t3105, t905, t606, t1052, t360);
    (t11554, t11560, t11574, t11627, t11631, t11656, t11661, t11670)
}
