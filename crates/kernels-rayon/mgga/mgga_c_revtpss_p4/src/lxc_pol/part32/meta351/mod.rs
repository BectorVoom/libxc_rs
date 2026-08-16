//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta351 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1287;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1288;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta351(t14370: f64, t4401: f64, t4391: f64, t705: f64, t2615: f64, t4311: f64, t1469: f64, t2609: f64, t706: f64, t1568: f64, t785: f64, t780: f64, t2439: f64, t212: f64, t4469: f64, t689: f64, t1579: f64, t2769: f64, t886: f64, t252: f64, t2782: f64, t2470: f64, t4480: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14372, t14386, t14433, t14441, t14473) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1287(t14370, t4401, t4391, t705, t2615, t4311, t1469, t2609, t706, t1568, t785, t780);
        let (t14474, t14479, t14481, t14484, t14485) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1288(t14473, t2439, t212, t4469, t780, t689, t1579, t2769, t886, t252, t2782, t2470, t4480);
    (t14372, t14386, t14433, t14441, t14474, t14479, t14481, t14484, t14485)
}
