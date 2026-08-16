//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta278 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1134;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1135;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1136;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta278(t11876: f64, t3117: f64, t1016: f64, t697: f64, t1011: f64, t1010: f64, t2270: f64, t3241: f64, t3244: f64, t1058: f64, t3197: f64, t11132: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11147: f64, t11153: f64, t11158: f64, t11162: f64, t11167: f64, t11171: f64, t341: f64, t225: f64, t366: f64, t1053: f64, t3196: f64, t11151: f64, t247: f64, t3182: f64, t3163: f64, t3172: f64, t3161: f64, t1017: f64, t1063: f64, t11855: f64, t11859: f64, t11862: f64, t11866: f64, t11871: f64, t11875: f64, t3101: f64, t3115: f64, t3120: f64, t3188: f64, t375: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11877, t11880, t11881, t11883, t11886, t11888, t11890) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1134(t11876, t3117, t1016, t697, t1011, t1010, t2270, t3241, t3244, t1058, t3197, t11132);
        let (t11901, t11902) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1135(t11134, t11136, t11138, t11140, t11147, t11153, t11158, t11162, t11167, t11171, t11890, t341);
        let (t11903, t11904, t11907, t11913, t11916, t11919) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1136(t11902, t225, t366, t1053, t3196, t11151, t247, t3182, t3163, t3172, t3161, t1017, t1063, t11855, t11859, t11862, t11866, t11871, t11875, t11877, t11881, t11883, t11886, t11888, t3101, t3115, t3120, t3188, t375);
    (t11877, t11880, t11883, t11901, t11902, t11903, t11904, t11907, t11913, t11916, t11919)
}
