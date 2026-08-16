//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta483 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1768;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1769;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta483(t2467: f64, t25399: f64, t233: f64, t867: f64, t1949: f64, t7056: f64, t10073: f64, t1957: f64, t822: f64, t25386: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t25400, t25402, t25403, t25404, t25406, t25410) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1768(t2467, t25399, t233, t867, t1949, t7056, t10073, t1957, t822);
        let t25411 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1769(t25386, t25410);
    (t25400, t25402, t25403, t25404, t25406, t25410, t25411)
}
