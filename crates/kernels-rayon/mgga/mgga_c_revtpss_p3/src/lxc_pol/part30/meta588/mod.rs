//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta588 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2046;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2047;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta588(t136: f64, t2457: f64, t7307: f64, t25944: f64, t26035: f64, t686: f64, t72: f64, t7284: f64, t25878: f64, t94597: f64, t10073: f64, t25937: f64, t7274: f64, t7282: f64, t1955: f64, t9656: f64, t25904: f64, t94634: f64, t94640: f64, t281: f64, t555: f64, t93238: f64, t25898: f64, t7303: f64, t25917: f64, t9303: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94806, t94807, t94810, t94811, t94813, t94820) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2046(t136, t2457, t7307, t25944, t26035, t686, t72, t7284, t25878, t94597, t10073, t25937, t7274, t7282);
        let (t94823, t94842, t94844, t94849, t94851, t94854) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2047(t1955, t7282, t9656, t25904, t94634, t94640, t281, t555, t93238, t25898, t7303, t25917, t9303);
    (t94806, t94807, t94810, t94811, t94813, t94820, t94823, t94842, t94844, t94849, t94851, t94854)
}
