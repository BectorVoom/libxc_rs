//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1754;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1755;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1756;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta479(t25299: f64, t25301: f64, t1954: f64, t9645: f64, t7057: f64, t1032: f64, t860: f64, t867: f64, t786: f64, t7060: f64, t11007: f64, t233: f64, t213: f64, t7048: f64, t2470: f64, t7059: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25303, t25304) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1754(t25299, t25301, t1954, t9645);
        let (t25305, t25307, t25308, t25309, t25310, t25311, t25317) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1755(t25304, t7057, t25301, t1032, t860, t867, t786, t7060, t11007, t233);
        let (t25322, t25331) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1756(t213, t7048, t2470, t7059);
    (t25303, t25304, t25305, t25307, t25308, t25309, t25310, t25311, t25317, t25322, t25331)
}
