//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta468 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1780;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1781;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta468(t25304: f64, t7057: f64, t25301: f64, t1032: f64, t860: f64, t867: f64, t786: f64, t7060: f64, t233: f64, t25286: f64, t1957: f64, t11007: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25305, t25307, t25308, t25309) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1780(t25304, t7057, t25301, t1032, t860, t867);
        let (t25310, t25311, t25313, t25314, t25317) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1781(t25309, t786, t7060, t233, t25286, t1957, t11007);
    (t25305, t25307, t25308, t25309, t25310, t25311, t25313, t25314, t25317)
}
