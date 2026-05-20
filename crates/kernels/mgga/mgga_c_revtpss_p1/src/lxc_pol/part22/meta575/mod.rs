//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2429;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2430;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2431;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta575<F: Float>(t2741: F, t6019: F, t5966: F, t775: F, t10698: F, t828: F, t1544: F, t4343: F, t2477: F, t5984: F, t800: F, t5988: F, t1548: F, t10811: F, t6037: F, t18444: F, t4364: F, t4366: F, t10846: F, t10885: F, t10888: F, t10891: F, t10900: F, t2730: F, t4362: F, t851: F, t10871: F, t836: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t18491, t18493, t18495, t18498, t18500, t18507, t18511) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2429::<F>(t2741, t6019, t5966, t775, t10698, t828, t1544, t4343, t2477, t5984, t800, t5988);
        let (t18515, t18521, t18524) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2430::<F>(t1548, t4343, t800, t10811, t6037, t18444, t4364, t4366, t10846, t10885, t10888, t10891, t10900, t18491, t18495, t18500, t18507, t18511, t2730, t4362, t851);
        let t18525 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2431::<F>(t10871, t836);
    (t18493, t18495, t18498, t18500, t18507, t18511, t18515, t18521, t18524, t18525)
}
