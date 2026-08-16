//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta212 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk832;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk833;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk834;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta212(t4039: f64, t4032: f64, t4024: f64, t3854: f64, t3859: f64, t3862: f64, t3867: f64, t3871: f64, t3873: f64, t4030: f64, t4035: f64, t4037: f64, t4042: f64, t225: f64, t5638: f64, t539: f64, t73: f64, t1412: f64, t1868: f64, t1353: f64, t1394: f64, t5591: f64, t1392: f64, t1395: f64, t1877: f64, t1879: f64, t541: f64, t543: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5639, t5640, t5641, t5642) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk832(t4039, t4032, t4024, t3854, t3859, t3862, t3867, t3871, t3873, t4030, t4035, t4037, t4042);
        let (t5644, t5650, t5651, t5652, t5655, t5658) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk833(t225, t5638, t5642, t539, t73, t1412, t1868, t1353, t1394, t5591, t1392, t1395, t1877, t1879, t541);
        let t5659 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk834(t543, t5658);
    (t5639, t5640, t5641, t5644, t5650, t5651, t5652, t5655, t5658, t5659)
}
