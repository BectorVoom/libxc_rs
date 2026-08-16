//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2013;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2014;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta411(t14239: f64, t4104: f64, t2470: f64, t5740: f64, t4101: f64, t1432: f64, t5763: f64, t1385: f64, t5710: f64, t10105: f64, t10109: f64, t10114: f64, t10117: f64, t10120: f64, t10126: f64, t10129: f64, t10137: f64, t10143: f64, t13921: f64, t1399: f64, t1437: f64, t3924: f64, t4118: f64, t5659: f64, t5767: f64, t820: f64, t14151: f64, t14200: f64, t14237: f64, t1427: f64, t1904: f64, t3899: f64, t689: f64, t10151: f64, t10154: f64, t14091: f64, t14096: f64, t14097: f64, t14102: f64, t14105: f64, t14108: f64, t14111: f64, t1424: f64, t4132: f64, t5715: f64, t9695: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14241, t14242, t14243, t14252, t14255, t14266) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2013(t14239, t4104, t2470, t5740, t4101, t1432, t5763, t1385, t5710, t10105, t10109, t10114, t10117, t10120, t10126, t10129, t10137, t10143, t13921, t1399, t1437, t3924, t4118, t5659, t5767, t820);
        let (t14268, t14269, t14274, t14276, t14279) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2014(t14151, t14200, t14237, t14266, t1427, t1904, t3899, t689, t10151, t10154, t14091, t14096, t14097, t14102, t14105, t14108, t14111, t1424, t4132, t5715, t9695);
    (t14241, t14242, t14243, t14252, t14255, t14268, t14269, t14274, t14276, t14279)
}
