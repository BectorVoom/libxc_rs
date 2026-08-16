//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta309 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1297;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1298;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta309(t10001: f64, t10003: f64, t1419: f64, t4086: f64, t786: f64, t4104: f64, t268: f64, t4056: f64, t543: f64, t675: f64, t4101: f64, t555: f64, t5744: f64, t3923: f64, t4003: f64, t2435: f64, t4093: f64, t4083: f64, t9303: f64, t4066: f64, t545: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10004, t10014, t10015, t10020, t10022) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1297(t10001, t10003, t1419, t4086, t786, t4104, t268, t4056, t543, t675, t4101, t555, t5744);
        let (t10024, t10027, t10032, t10035, t10039) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1298(t10022, t786, t3923, t675, t268, t4003, t2435, t4093, t4083, t9303, t4066, t545);
    (t10004, t10014, t10015, t10020, t10022, t10024, t10027, t10032, t10035, t10039)
}
