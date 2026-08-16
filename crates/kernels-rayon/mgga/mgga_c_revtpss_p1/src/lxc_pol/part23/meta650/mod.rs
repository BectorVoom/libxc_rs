//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta650 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2376;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2377;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta650(t212: f64, t2237: f64, t225: f64, t816: f64, t2665: f64, t10689: f64, t237: f64, t247: f64, t2783: f64, t9801: f64, t10745: f64, t2735: f64, t4503: f64, t2693: f64, t2710: f64, t9732: f64, t2682: f64, t820: f64, t823: f64, t10292: f64, t65: f64, t235: f64, t826: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40488, t40489, t40507, t40517, t40518, t40521) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2376(t212, t2237, t225, t816, t2665, t10689, t237, t247, t2783, t9801, t10745, t2735, t4503);
        let (t40535, t40593, t40604, t40607, t40609) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2377(t2693, t2710, t9732, t2682, t820, t823, t10292, t65, t235, t826, t225, t785);
    (t40488, t40489, t40507, t40517, t40518, t40521, t40535, t40593, t40604, t40607, t40609)
}
