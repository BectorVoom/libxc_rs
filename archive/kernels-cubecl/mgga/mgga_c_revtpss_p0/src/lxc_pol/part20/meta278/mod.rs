//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta278 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1134;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1135;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1136;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta278<F: Float>(t11876: F, t3117: F, t1016: F, t697: F, t1011: F, t1010: F, t2270: F, t3241: F, t3244: F, t1058: F, t3197: F, t11132: F, t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11158: F, t11162: F, t11167: F, t11171: F, t341: F, t225: F, t366: F, t1053: F, t3196: F, t11151: F, t247: F, t3182: F, t3163: F, t3172: F, t3161: F, t1017: F, t1063: F, t11855: F, t11859: F, t11862: F, t11866: F, t11871: F, t11875: F, t3101: F, t3115: F, t3120: F, t3188: F, t375: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11877, t11880, t11881, t11883, t11886, t11888, t11890) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1134::<F>(t11876, t3117, t1016, t697, t1011, t1010, t2270, t3241, t3244, t1058, t3197, t11132);
        let (t11901, t11902) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1135::<F>(t11134, t11136, t11138, t11140, t11147, t11153, t11158, t11162, t11167, t11171, t11890, t341);
        let (t11903, t11904, t11907, t11913, t11916, t11919) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1136::<F>(t11902, t225, t366, t1053, t3196, t11151, t247, t3182, t3163, t3172, t3161, t1017, t1063, t11855, t11859, t11862, t11866, t11871, t11875, t11877, t11881, t11883, t11886, t11888, t3101, t3115, t3120, t3188, t375);
    (t11877, t11880, t11883, t11901, t11902, t11903, t11904, t11907, t11913, t11916, t11919)
}
