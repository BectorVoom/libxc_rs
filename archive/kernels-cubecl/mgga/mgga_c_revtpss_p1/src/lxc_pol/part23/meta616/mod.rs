//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2290;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2291;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta616<F: Float>(t24543: F, t482: F, t13063: F, t1042: F, t22700: F, t344: F, t1261: F, t13062: F, t17377: F, t17529: F, t17569: F, t17572: F, t1808: F, t20784: F, t20787: F, t20789: F, t21143: F, t21272: F, t24535: F, t464: F, t5274: F, t5391: F, t6619: F, t6625: F, t6631: F, t6635: F, t6673: F, t12839: F, t1469: F, t20795: F, t3626: F, t6638: F, t17304: F, t17340: F, t17342: F, t17438: F, t1791: F, t20817: F, t20843: F, t20847: F, t20851: F, t20917: F, t20927: F, t20966: F, t21177: F, t5331: F, t5340: F, t6611: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t24544, t24545, t24546, t24551, t24562) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2290::<F>(t24543, t482, t13063, t1042, t22700, t344, t1261, t13062, t17377, t17529, t17569, t17572, t1808, t20784, t20787, t20789, t21143, t21272, t24535, t464, t5274, t5391, t6619, t6625, t6631, t6635, t6673);
        let (t24568, t24569, t24572, t24573, t24587) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2291::<F>(t12839, t1469, t20795, t3626, t6638, t17304, t17340, t17342, t17438, t1791, t20817, t20843, t20847, t20851, t20917, t20927, t20966, t21177, t5331, t5340, t6611);
    (t24544, t24545, t24546, t24551, t24562, t24568, t24569, t24572, t24573, t24587)
}
