//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta501 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1888;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1889;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1890;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta501<F: Float>(t25304: F, t7283: F, t25946: F, t25949: F, t786: F, t7286: F, t225: F, t26034: F, t1426: F, t3999: F, t26044: F, t4003: F, t213: F, t7274: F, t1445: F, t2027: F, t25921: F, t25961: F, t25966: F, t26036: F, t26040: F, t26043: F, t26046: F, t26051: F, t26055: F, t26058: F, t26062: F, t26065: F, t26067: F, t4078: F, t561: F, t7279: F, t7295: F, t7304: F, t25956: F, t532: F, t1450: F, t2014: F, t118: F, t2011: F, t2322: F, t2331: F, t2372: F, t25800: F, t25804: F, t25805: F, t25835: F, t25838: F, t25840: F, t25842: F, t25844: F, t25846: F, t25853: F, t25858: F, t25860: F, t25863: F, t25868: F, t25872: F, t4151: F, t569: F, t651: F, t671: F, t6985: F, t7007: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26069, t26071, t26072, t26073, t26075, t26079, t26080) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1888::<F>(t25304, t7283, t25946, t25949, t786, t7286, t225, t26034, t1426, t3999, t26044, t4003);
        let (t26081, t26084, t26087) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1889::<F>(t26079, t26080, t213, t7274, t1445, t2027, t25921, t25961, t25966, t26036, t26040, t26043, t26046, t26051, t26055, t26058, t26062, t26065, t26067, t26071, t26073, t26075, t4078, t561, t7279, t7295, t7304);
        let (t26088, t26089, t26090, t26092) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1890::<F>(t25956, t26087, t532, t1450, t2014, t118, t2011, t2322, t2331, t2372, t25800, t25804, t25805, t25835, t25838, t25840, t25842, t25844, t25846, t25853, t25858, t25860, t25863, t25868, t25872, t4151, t569, t651, t671, t6985, t7007);
    (t26069, t26071, t26072, t26073, t26075, t26079, t26081, t26084, t26088, t26089, t26090, t26092)
}
