//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta558 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2385;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2386;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta558<F: Float>(t17883: F, t5351: F, t1269: F, t3766: F, t460: F, t1280: F, t17345: F, t1287: F, t17389: F, t17600: F, t1248: F, t5412: F, t1204: F, t12723: F, t1281: F, t1285: F, t1288: F, t12987: F, t17289: F, t17307: F, t17861: F, t17864: F, t17869: F, t17876: F, t17880: F, t1825: F, t3552: F, t3666: F, t3751: F, t3755: F, t3782: F, t5449: F, t5459: F, t5466: F, t5478: F, t5481: F, t5494: F) -> (F, F, F, F, F, F, F, F) {
        let (t17884, t17887, t17888, t17893, t17902, t17905, t17909) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2385::<F>(t17883, t5351, t1269, t3766, t460, t1280, t17345, t1287, t17389, t17600, t1248, t5412);
        let t17912 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2386::<F>(t1204, t12723, t1281, t1285, t1288, t12987, t17289, t17307, t17861, t17864, t17869, t17876, t17880, t17884, t17888, t17893, t17902, t17905, t17909, t1825, t3552, t3666, t3751, t3755, t3782, t5449, t5459, t5466, t5478, t5481, t5494);
    (t17884, t17887, t17888, t17893, t17902, t17905, t17909, t17912)
}
