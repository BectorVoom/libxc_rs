//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta391 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1416;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta391(t17330: f64, t459: f64, t225: f64, t480: f64, t1256: f64, t5258: f64, t5262: f64, t1804: f64, t3655: f64, t1786: f64, t1260: f64, t12987: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t17331, t17333, t17337, t17339, t17340, t17342, t17344) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1416(t17330, t459, t225, t480, t1256, t5258, t5262, t1804, t3655, t1786, t1260, t12987);
    (t17331, t17333, t17337, t17339, t17340, t17342, t17344)
}
