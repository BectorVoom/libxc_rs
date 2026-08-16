//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta348 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1368;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta348(t11858: f64, t4891: f64, t1086: f64, t3046: f64, t3090: f64, t3316: f64, t994: f64, t1016: f64, t697: f64, t1011: f64, t1010: f64, t2270: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11859, t11865, t11866, t11874, t11875, t11880, t11881, t11883) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1368(t11858, t4891, t1086, t3046, t3090, t3316, t994, t1016, t697, t1011, t1010, t2270);
    (t11859, t11865, t11866, t11874, t11875, t11880, t11881, t11883)
}
