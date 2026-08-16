//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta501 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1888;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1889;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1890;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta501(t25304: f64, t7283: f64, t25946: f64, t25949: f64, t786: f64, t7286: f64, t225: f64, t26034: f64, t1426: f64, t3999: f64, t26044: f64, t4003: f64, t213: f64, t7274: f64, t1445: f64, t2027: f64, t25921: f64, t25961: f64, t25966: f64, t26036: f64, t26040: f64, t26043: f64, t26046: f64, t26051: f64, t26055: f64, t26058: f64, t26062: f64, t26065: f64, t26067: f64, t4078: f64, t561: f64, t7279: f64, t7295: f64, t7304: f64, t25956: f64, t532: f64, t1450: f64, t2014: f64, t118: f64, t2011: f64, t2322: f64, t2331: f64, t2372: f64, t25800: f64, t25804: f64, t25805: f64, t25835: f64, t25838: f64, t25840: f64, t25842: f64, t25844: f64, t25846: f64, t25853: f64, t25858: f64, t25860: f64, t25863: f64, t25868: f64, t25872: f64, t4151: f64, t569: f64, t651: f64, t671: f64, t6985: f64, t7007: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26069, t26071, t26072, t26073, t26075, t26079, t26080) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1888(t25304, t7283, t25946, t25949, t786, t7286, t225, t26034, t1426, t3999, t26044, t4003);
        let (t26081, t26084, t26087) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1889(t26079, t26080, t213, t7274, t1445, t2027, t25921, t25961, t25966, t26036, t26040, t26043, t26046, t26051, t26055, t26058, t26062, t26065, t26067, t26071, t26073, t26075, t4078, t561, t7279, t7295, t7304);
        let (t26088, t26089, t26090, t26092) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1890(t25956, t26087, t532, t1450, t2014, t118, t2011, t2322, t2331, t2372, t25800, t25804, t25805, t25835, t25838, t25840, t25842, t25844, t25846, t25853, t25858, t25860, t25863, t25868, t25872, t4151, t569, t651, t671, t6985, t7007);
    (t26069, t26071, t26072, t26073, t26075, t26079, t26081, t26084, t26088, t26089, t26090, t26092)
}
