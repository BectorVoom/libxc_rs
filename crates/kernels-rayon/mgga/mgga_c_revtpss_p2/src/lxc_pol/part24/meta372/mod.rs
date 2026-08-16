//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta372 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1262;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1263;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta372(t1277: f64, t24514: f64, t1774: f64, t3737: f64, t6702: f64, t1828: f64, t13182: f64, t13100: f64, t24228: f64, t247: f64, t1794: f64, t6628: f64, t482: f64, t13063: f64, t1042: f64, t22700: f64, t344: f64, t1261: f64, t13062: f64, t17377: f64, t17529: f64, t17569: f64, t17572: f64, t1808: f64, t20784: f64, t20787: f64, t20789: f64, t21143: f64, t21272: f64, t464: f64, t5274: f64, t5391: f64, t6619: f64, t6625: f64, t6631: f64, t6635: f64, t6673: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24515, t24519, t24524, t24525, t24535, t24543) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1262(t1277, t24514, t1774, t3737, t6702, t1828, t13182, t13100, t24228, t247, t1794, t6628);
        let (t24544, t24545, t24546, t24551, t24562) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1263(t24543, t482, t13063, t1042, t22700, t344, t1261, t13062, t17377, t17529, t17569, t17572, t1808, t20784, t20787, t20789, t21143, t21272, t24535, t464, t5274, t5391, t6619, t6625, t6631, t6635, t6673);
    (t24515, t24519, t24524, t24525, t24535, t24543, t24544, t24545, t24546, t24551, t24562)
}
