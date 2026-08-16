//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta291 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1159;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1160;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1161;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1162;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta291(t12108: f64, t12172: f64, t1079: f64, t1096: f64, t3059: f64, t1073: f64, t1076: f64, t1097: f64, t11220: f64, t11224: f64, t11902: f64, t12034: f64, t12040: f64, t12043: f64, t3043: f64, t3047: f64, t3052: f64, t3058: f64, t3060: f64, t3063: f64, t3067: f64, t3076: f64, t3264: f64, t3271: f64, t3326: f64, t342: f64, t386: f64, t995: f64, t11217: f64, t1100: f64, t1102: f64, t11105: f64, t11108: f64, t11114: f64, t11118: f64, t11398: f64, t11530: f64, t11533: f64, t11547: f64, t11608: f64, t11612: f64, t11614: f64, t11618: f64, t198: f64, t3329: f64, t3336: f64, t336: f64, t5023: f64, t11291: f64, t11293: f64, t11296: f64, t11303: f64, t11382: f64, t11390: f64, t11392: f64, t11394: f64, t11590: f64, t11593: f64, t11596: f64, t11600: f64, t11604: f64, t30: f64, t265: f64, t393: f64, t11095: f64, t10326: f64, t1106: f64, t2257: f64, t2258: f64, t2838: f64, t3340: f64, t395: f64, t45: f64, t605: f64, t606: f64, t895: f64, t9344: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t12173, t12174, t12178, t12189) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1159(t12108, t12172, t1079, t1096, t3059, t1073, t1076, t1097, t11220, t11224, t11902, t12034, t12040, t12043, t3043, t3047, t3052, t3058, t3060, t3063, t3067, t3076, t3264, t3271, t3326, t342, t386, t995);
        let (t12190, t12198) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1160(t11217, t12189, t1100, t1102, t11105, t11108, t11114, t11118, t11398, t11530, t11533, t11547, t11608, t11612, t11614, t11618, t198, t3329, t3336, t336, t5023);
        let t12199 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1161(t11291, t11293, t11296, t11303, t11382, t11390, t11392, t11394, t11590, t11593, t11596, t11600, t11604);
        let (t12201, t12211) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1162(t30, t265, t393, t11095, t12198, t12199, t10326, t1106, t2257, t2258, t2838, t3340, t395, t45, t605, t606, t895, t9344, dens_threshold, rho0, zeta_threshold);
    (t12173, t12174, t12178, t12190, t12201, t12211)
}
