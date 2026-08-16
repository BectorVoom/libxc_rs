//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta383 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1805;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1806;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1807;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1808;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1809;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1810;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1811;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta383(t12295: f64, t12351: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12307: f64, t12310: f64, t12314: f64, t12317: f64, t12320: f64, t12344: f64, t12347: f64, t12354: f64, t12531: f64, t1188: f64, t1178: f64, t3519: f64, t439: f64, t3522: f64, t447: f64, t12487: f64, t1161: f64, t1180: f64, t1189: f64, t12429: f64, t12431: f64, t12465: f64, t12470: f64, t12473: f64, t12476: f64, t12481: f64, t12486: f64, t12488: f64, t12491: f64, t12494: f64, t12497: f64, t12501: f64, t12504: f64, t12508: f64, t12511: f64, t12514: f64, t3452: f64, t3454: f64, t3477: f64, t3491: f64, t3496: f64, t3498: f64, t3516: f64, t3521: f64, t3524: f64, t12426: f64, t300: f64, t12224: f64, t12233: f64, t12237: f64, t12240: f64, t12242: f64, t12245: f64, t12251: f64, t12360: f64, t12363: f64, t12366: f64, t12381: f64, t12395: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12542, t12543, t12546) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1805(t12295, t12351, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320, t12344, t12347, t12354);
        let t12547 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1806(t12531, t12546);
        let (t12548, t12552) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1807(t1188, t12547, t1178, t3519);
        let t12553 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1808(t12552, t439);
        let t12555 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1809(t3522, t447);
        let (t12556, t12559) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1810(t12487, t12555, t1161, t1180, t1189, t12429, t12431, t12465, t12470, t12473, t12476, t12481, t12486, t12488, t12491, t12494, t12497, t12501, t12504, t12508, t12511, t12514, t12548, t12553, t3452, t3454, t3477, t3491, t3496, t3498, t3516, t3521, t3524);
        let (t12561, t12562) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1811(t12426, t12559, t300, t12224, t12233, t12237, t12240, t12242, t12245, t12251, t12360, t12363, t12366, t12381, t12395);
    (t12542, t12543, t12547, t12548, t12552, t12553, t12555, t12556, t12561, t12562)
}
