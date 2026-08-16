//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta467 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1777;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1778;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1779;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta467(t225: f64, t25286: f64, t7048: f64, t7071: f64, t886: f64, t7082: f64, t72: f64, t686: f64, t7058: f64, t2453: f64, t7057: f64, t136: f64, t1958: f64, t2457: f64, t1954: f64, t9645: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25287, t25292, t25295, t25296) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1777(t225, t25286, t7048, t7071, t886, t7082, t72, t686);
        let (t25297, t25299, t25300, t25301) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1778(t25296, t7058, t2453, t7057, t136, t1958, t2457);
        let (t25303, t25304) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1779(t25299, t25301, t1954, t9645);
    (t25287, t25292, t25295, t25296, t25297, t25299, t25300, t25301, t25303, t25304)
}
