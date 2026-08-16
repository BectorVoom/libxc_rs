//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta164 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk814;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk815;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk816;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk817;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta164(t448: f64, t6513: f64, t1756: f64, t1188: f64, t3503: f64, t3510: f64, t5044: f64, t5093: f64, t6423: f64, t6427: f64, t6431: f64, t6443: f64, t6450: f64, t6456: f64, t6458: f64, t6462: f64, t6465: f64, t6468: f64, t3523: f64, t1161: f64, t1180: f64, t1745: f64, t1757: f64, t3452: f64, t3477: f64, t3496: f64, t3521: f64, t435: f64, t5120: f64, t5158: f64, t6435: f64, t6437: f64, t6441: f64, t6473: f64, t6476: f64, t6481: f64, t6487: f64, t6503: f64, t6506: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t6514, t6518) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk814(t448, t6513, t1756);
        let (t6519, t6534) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk815(t1188, t6518, t3503, t3510, t5044, t5093, t6423, t6427, t6431, t6443, t6450, t6456, t6458, t6462, t6465, t6468);
        let t6535 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk816(t1188, t6534);
        let (t6538, t6541) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk817(t3523, t6518, t1161, t1180, t1745, t1757, t3452, t3477, t3496, t3521, t435, t5120, t5158, t6435, t6437, t6441, t6473, t6476, t6481, t6487, t6503, t6506, t6514, t6519, t6535);
    (t6514, t6518, t6519, t6534, t6535, t6538, t6541)
}
