//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2290;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2291;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta616(t24543: f64, t482: f64, t13063: f64, t1042: f64, t22700: f64, t344: f64, t1261: f64, t13062: f64, t17377: f64, t17529: f64, t17569: f64, t17572: f64, t1808: f64, t20784: f64, t20787: f64, t20789: f64, t21143: f64, t21272: f64, t24535: f64, t464: f64, t5274: f64, t5391: f64, t6619: f64, t6625: f64, t6631: f64, t6635: f64, t6673: f64, t12839: f64, t1469: f64, t20795: f64, t3626: f64, t6638: f64, t17304: f64, t17340: f64, t17342: f64, t17438: f64, t1791: f64, t20817: f64, t20843: f64, t20847: f64, t20851: f64, t20917: f64, t20927: f64, t20966: f64, t21177: f64, t5331: f64, t5340: f64, t6611: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24544, t24545, t24546, t24551, t24562) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2290(t24543, t482, t13063, t1042, t22700, t344, t1261, t13062, t17377, t17529, t17569, t17572, t1808, t20784, t20787, t20789, t21143, t21272, t24535, t464, t5274, t5391, t6619, t6625, t6631, t6635, t6673);
        let (t24568, t24569, t24572, t24573, t24587) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2291(t12839, t1469, t20795, t3626, t6638, t17304, t17340, t17342, t17438, t1791, t20817, t20843, t20847, t20851, t20917, t20927, t20966, t21177, t5331, t5340, t6611);
    (t24544, t24545, t24546, t24551, t24562, t24568, t24569, t24572, t24573, t24587)
}
