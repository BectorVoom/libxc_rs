//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta372 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1262;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1263;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta372<F: Float>(t1277: F, t24514: F, t1774: F, t3737: F, t6702: F, t1828: F, t13182: F, t13100: F, t24228: F, t247: F, t1794: F, t6628: F, t482: F, t13063: F, t1042: F, t22700: F, t344: F, t1261: F, t13062: F, t17377: F, t17529: F, t17569: F, t17572: F, t1808: F, t20784: F, t20787: F, t20789: F, t21143: F, t21272: F, t464: F, t5274: F, t5391: F, t6619: F, t6625: F, t6631: F, t6635: F, t6673: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t24515, t24519, t24524, t24525, t24535, t24543) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1262::<F>(t1277, t24514, t1774, t3737, t6702, t1828, t13182, t13100, t24228, t247, t1794, t6628);
        let (t24544, t24545, t24546, t24551, t24562) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1263::<F>(t24543, t482, t13063, t1042, t22700, t344, t1261, t13062, t17377, t17529, t17569, t17572, t1808, t20784, t20787, t20789, t21143, t21272, t24535, t464, t5274, t5391, t6619, t6625, t6631, t6635, t6673);
    (t24515, t24519, t24524, t24525, t24535, t24543, t24544, t24545, t24546, t24551, t24562)
}
