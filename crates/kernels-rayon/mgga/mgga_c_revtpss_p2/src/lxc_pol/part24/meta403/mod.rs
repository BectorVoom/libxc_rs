//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1339;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta403(t10868: f64, t820: f64, t843: f64, t2482: f64, t27: f64, t823: f64, t9948: f64, t2681: f64, t2719: f64, t10111: f64, t9720: f64, t2237: f64, t849: f64, t242: f64, t240: f64, t72: f64, t212: f64, t225: f64, t816: f64, t10689: f64, t237: f64, t247: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40348, t40352, t40360, t40398, t40406, t40424) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1339(t10868, t820, t843, t2482, t27, t823, t9948, t2681, t2719, t10111, t9720, t2237);
        let (t40452, t40462, t40488, t40507) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1340(t10111, t849, t9720, t242, t240, t72, t212, t2237, t225, t816, t10689, t237, t247);
    (t40348, t40352, t40360, t40398, t40406, t40424, t40452, t40462, t40488, t40507)
}
