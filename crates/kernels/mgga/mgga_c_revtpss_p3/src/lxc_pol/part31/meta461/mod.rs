//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1691;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1692;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1693;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1694;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1695;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1696;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1697;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1698;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta461<F: Float>(t21829: F, t665: F, t10227: F, t5895: F, t658: F, t1504: F, t2: F, t580: F, t2349: F, t5823: F, t9342: F, t100: F, t10241: F, t5907: F, t661: F, t1509: F, t2357: F, t5911: F, t108: F, t105: F, t13475: F, t13496: F, t1507: F, t4280: F, t4284: F, t5896: F, t5899: F, t5902: F, t656: F, t662: F, t97: F, t655: F, t10201: F, t10202: F, t13448: F, t13451: F, t13453: F, t21818: F, t21821: F, t21824: F, t21827: F, t69: F, t114: F, t30: F, t508: F, t1518: F, t5517: F, t13584: F, t9375: F, t6785: F, t9335: F, t3833: F, t5824: F, t18280: F, t2255: F, t513: F, t5549: F, t605: F, zeta_threshold: F, t33: F, t6792: F, t9350: F, t3841: F, t6416: F, t1113: F, t20256: F, t516: F, t5557: F, t162: F, t187: F, t1450: F, t6922: F, t9605: F, t3874: F, t1344: F, t5574: F, t9617: F, t3881: F, t1348: F, t5582: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t21830, t21836, t21840, t21846, t21850, t21851) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1691::<F>(t21829, t665, t10227, t5895, t658, t1504, t2, t580, t2349, t5823, t9342, t100);
        let t21876 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1692::<F>(t10241, t5907, t661, t1509, t2, t580, t2357, t5911, t21850, t108, t105, t13475, t13496, t1507, t21836, t21840, t21846, t21851, t4280, t4284, t5896, t5899, t5902, t656, t662, t97);
        let t21880 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1693::<F>(t21876, t655, t10201, t10202, t13448, t13451, t13453, t21818, t21821, t21824, t21827, t21830, t69);
        let t21881 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1694::<F>(t114, t21880);
        let (t21882, t21891, t21901, t21905, t21917) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1695::<F>(t30, t21881, t508, t1518, t5517, t13584, t9375, t6785, t9335, t3833, t5824, t18280, t2255, t513, t5549, t605, zeta_threshold);
        let (t21931, t21933) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1696::<F>(t33, t6792, t9350, t3841, t6416, t1113, t20256, t2255, t516, t5557, t162, t21917, t187, zeta_threshold);
        let (t21937, t21955, t21956) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1697::<F>(t30, t1450, t6922, t6785, t9605, t3874, t5824, t1344, t18280, t2255, t5574, t605, t6792, t9617, zeta_threshold);
        let t21969 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1698::<F>(t33, t3881, t6416, t1113, t1348, t20256, t21956, t2255, t5582, t21955, zeta_threshold);
    (t21876, t21881, t21882, t21891, t21901, t21905, t21931, t21933, t21937, t21969)
}
