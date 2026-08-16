//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta209 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk828;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta209(t1892: f64, t212: f64, t1358: f64, t689: f64, t1893: f64, t786: f64, t1364: f64, t1889: f64, t3989: f64, t1882: f64, t550: f64, t543: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5599, t5600, t5601, t5603, t5604, t5606, t5608, t5609) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk828(t1892, t212, t1358, t689, t1893, t786, t1364, t1889, t3989, t1882, t550, t543);
    (t5599, t5600, t5601, t5603, t5604, t5606, t5608, t5609)
}
