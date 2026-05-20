//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta560 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2390;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2391;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2392;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2393;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta560<F: Float>(t1234: F, t12699: F, t12709: F, t12717: F, t12723: F, t1285: F, t17331: F, t1770: F, t17917: F, t17921: F, t17934: F, t17941: F, t17945: F, t17949: F, t17952: F, t17955: F, t17958: F, t1822: F, t3670: F, t3746: F, t3756: F, t3770: F, t3774: F, t3778: F, t3787: F, t490: F, t5436: F, t5446: F, t5466: F, t5470: F, t5491: F, t17186: F, t17859: F, t17912: F, t1277: F, t1828: F, t3738: F, t13182: F, t3566: F, t488: F, t1276: F, t1774: F, t3575: F, t17807: F, t225: F, t494: F, t1209: F) -> (F, F, F, F, F, F, F, F, F) {
        let t17961 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2390::<F>(t1234, t12699, t12709, t12717, t12723, t1285, t17331, t1770, t17917, t17921, t17934, t17941, t17945, t17949, t17952, t17955, t17958, t1822, t3670, t3746, t3756, t3770, t3774, t3778, t3787, t490, t5436, t5446, t5466, t5470, t5491);
        let (t17963, t17964, t17967, t17968, t17973) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2391::<F>(t17186, t17859, t17912, t17961, t1277, t1828, t3738, t13182, t3566, t488);
        let t17974 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2392::<F>(t1276, t1774);
        let (t17975, t17979, t17986) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2393::<F>(t17974, t3575, t17807, t225, t494, t1209, t488);
    (t17963, t17964, t17967, t17968, t17973, t17974, t17975, t17979, t17986)
}
