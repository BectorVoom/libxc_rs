//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta649 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2374;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2375;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta649(t10868: f64, t2482: f64, t27: f64, t820: f64, t823: f64, t9948: f64, t839: f64, t2681: f64, t2719: f64, t10111: f64, t9720: f64, t685: f64, t827: f64, t837: f64, t2237: f64, t2487: f64, t849: f64, t775: f64, t855: f64, t242: f64, t240: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40352, t40360, t40361, t40398, t40406, t40409) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2374(t10868, t2482, t27, t820, t823, t9948, t839, t2681, t2719, t10111, t9720, t685, t827, t837);
        let (t40424, t40425, t40452, t40455, t40462) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2375(t2237, t2482, t823, t2487, t10111, t849, t9720, t685, t775, t855, t242, t240, t72);
    (t40352, t40360, t40361, t40398, t40406, t40409, t40424, t40425, t40452, t40455, t40462)
}
