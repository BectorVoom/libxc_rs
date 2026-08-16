//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2016;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta568(t10997: f64, t93261: f64, t25404: f64, t40270: f64, t10510: f64, t25399: f64, t10115: f64, t1951: f64, t7058: f64, t92871: f64, t1032: f64, t11007: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t93262, t93272, t93273, t93276, t93278, t93279) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2016(t10997, t93261, t25404, t40270, t10510, t25399, t10115, t1951, t7058, t92871, t1032, t11007);
    (t93262, t93272, t93273, t93276, t93278, t93279)
}
