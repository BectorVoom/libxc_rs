//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta595 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2067;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2068;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta595(t2453: f64, t26053: f64, t9676: f64, t4078: f64, t689: f64, t7242: f64, t1358: f64, t2439: f64, t7274: f64, t785: f64, t26064: f64, t3920: f64, t1426: f64, t7275: f64, t786: f64, t3917: f64, t25953: f64, t26072: f64, t2435: f64, t25913: f64, t7289: f64, t94600: f64, t2028: f64, t3999: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94725, t94726, t94729, t94733, t94735) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2067(t2453, t26053, t9676, t4078, t689, t7242, t1358, t2439, t7274, t785, t26064, t3920);
        let (t94748, t94749, t94756, t94758, t94761, t94762) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2068(t1426, t7275, t786, t3917, t25953, t26072, t2435, t25913, t7289, t94600, t2028, t3999);
    (t94725, t94726, t94729, t94733, t94735, t94748, t94749, t94756, t94758, t94761, t94762)
}
