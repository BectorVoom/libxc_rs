//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta144 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk728;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta144(t2630: f64, t3869: f64, t1337: f64, t2619: f64, t514: f64, t517: f64, t1359: f64, t2435: f64, t555: f64, t785: f64, t1358: f64, t2439: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3871, t3873, t3874, t3881, t3894, t3895, t3896, t3898) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk728(t2630, t3869, t1337, t2619, t514, t517, t1359, t2435, t555, t785, t1358, t2439);
    (t3871, t3873, t3874, t3881, t3894, t3895, t3896, t3898)
}
