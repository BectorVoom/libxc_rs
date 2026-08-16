//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2013;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2014;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta411<F: Float>(t14239: F, t4104: F, t2470: F, t5740: F, t4101: F, t1432: F, t5763: F, t1385: F, t5710: F, t10105: F, t10109: F, t10114: F, t10117: F, t10120: F, t10126: F, t10129: F, t10137: F, t10143: F, t13921: F, t1399: F, t1437: F, t3924: F, t4118: F, t5659: F, t5767: F, t820: F, t14151: F, t14200: F, t14237: F, t1427: F, t1904: F, t3899: F, t689: F, t10151: F, t10154: F, t14091: F, t14096: F, t14097: F, t14102: F, t14105: F, t14108: F, t14111: F, t1424: F, t4132: F, t5715: F, t9695: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14241, t14242, t14243, t14252, t14255, t14266) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2013::<F>(t14239, t4104, t2470, t5740, t4101, t1432, t5763, t1385, t5710, t10105, t10109, t10114, t10117, t10120, t10126, t10129, t10137, t10143, t13921, t1399, t1437, t3924, t4118, t5659, t5767, t820);
        let (t14268, t14269, t14274, t14276, t14279) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2014::<F>(t14151, t14200, t14237, t14266, t1427, t1904, t3899, t689, t10151, t10154, t14091, t14096, t14097, t14102, t14105, t14108, t14111, t1424, t4132, t5715, t9695);
    (t14241, t14242, t14243, t14252, t14255, t14268, t14269, t14274, t14276, t14279)
}
