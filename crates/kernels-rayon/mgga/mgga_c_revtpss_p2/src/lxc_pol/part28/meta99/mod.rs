//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta99 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk630;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk631;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta99(t2251: f64, t2275: f64, t2258: f64, t48: f64, t59: f64, t60: f64, t239: f64, t64: f64, t2270: f64, t44: f64, t49: f64, t56: f64, t614: f64, t617: f64, t38: f64, t45: f64, t631: f64, t78: f64, t57: f64, t635: f64, t81: f64, t633: f64, t637: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2282, t2283, t2286, t2289, t2290, t2291) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk630(t2251, t2275, t2258, t48, t59, t60, t239, t64, t2270, t44, t49, t56, t614, t617);
        let (t2292, t2297, t2299, t2304, t2306, t2311) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk631(t2291, t38, t45, t631, t78, t57, t635, t81, t2251, t2258, t633, t637);
    (t2282, t2283, t2286, t2289, t2290, t2291, t2292, t2297, t2299, t2304, t2306, t2311)
}
