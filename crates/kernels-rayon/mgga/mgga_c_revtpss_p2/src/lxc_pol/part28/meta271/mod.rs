//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta271 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1215;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1216;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1217;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta271(t3: f64, t7318: f64, t1459: f64, t2042: f64, t116: f64, t1936: f64, param_d: f64, t670: f64, t572: f64, t117: f64, t7002: f64, t1461: f64, t2040: f64, t573: f64, t38: f64, t4173: f64, t1497: f64, t84: f64, t77: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7319, t7324, t7329, t7330) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1215(t3, t7318, t1459, t2042, t116, t1936, param_d);
        let (t7331, t7334, t7337, t7702) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1216(t670, t7330, t572, t117, t7002, t1461, t2040, t573, t7324, t7329, t38, t4173);
        let (t7705, t7706) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1217(t1497, t84, t77);
    (t7319, t7324, t7330, t7331, t7334, t7337, t7702, t7705, t7706)
}
