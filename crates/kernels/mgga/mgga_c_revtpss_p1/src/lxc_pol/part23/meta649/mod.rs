//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta649 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2374;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2375;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta649<F: Float>(t10868: F, t2482: F, t27: F, t820: F, t823: F, t9948: F, t839: F, t2681: F, t2719: F, t10111: F, t9720: F, t685: F, t827: F, t837: F, t2237: F, t2487: F, t849: F, t775: F, t855: F, t242: F, t240: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40352, t40360, t40361, t40398, t40406, t40409) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2374::<F>(t10868, t2482, t27, t820, t823, t9948, t839, t2681, t2719, t10111, t9720, t685, t827, t837);
        let (t40424, t40425, t40452, t40455, t40462) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2375::<F>(t2237, t2482, t823, t2487, t10111, t849, t9720, t685, t775, t855, t242, t240, t72);
    (t40352, t40360, t40361, t40398, t40406, t40409, t40424, t40425, t40452, t40455, t40462)
}
