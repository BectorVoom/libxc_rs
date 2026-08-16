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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1691;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1692;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1693;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1694;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1695;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1696;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1697;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1698;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta461(t21829: f64, t665: f64, t10227: f64, t5895: f64, t658: f64, t1504: f64, t2: f64, t580: f64, t2349: f64, t5823: f64, t9342: f64, t100: f64, t10241: f64, t5907: f64, t661: f64, t1509: f64, t2357: f64, t5911: f64, t108: f64, t105: f64, t13475: f64, t13496: f64, t1507: f64, t4280: f64, t4284: f64, t5896: f64, t5899: f64, t5902: f64, t656: f64, t662: f64, t97: f64, t655: f64, t10201: f64, t10202: f64, t13448: f64, t13451: f64, t13453: f64, t21818: f64, t21821: f64, t21824: f64, t21827: f64, t69: f64, t114: f64, t30: f64, t508: f64, t1518: f64, t5517: f64, t13584: f64, t9375: f64, t6785: f64, t9335: f64, t3833: f64, t5824: f64, t18280: f64, t2255: f64, t513: f64, t5549: f64, t605: f64, zeta_threshold: f64, t33: f64, t6792: f64, t9350: f64, t3841: f64, t6416: f64, t1113: f64, t20256: f64, t516: f64, t5557: f64, t162: f64, t187: f64, t1450: f64, t6922: f64, t9605: f64, t3874: f64, t1344: f64, t5574: f64, t9617: f64, t3881: f64, t1348: f64, t5582: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21830, t21836, t21840, t21846, t21850, t21851) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1691(t21829, t665, t10227, t5895, t658, t1504, t2, t580, t2349, t5823, t9342, t100);
        let t21876 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1692(t10241, t5907, t661, t1509, t2, t580, t2357, t5911, t21850, t108, t105, t13475, t13496, t1507, t21836, t21840, t21846, t21851, t4280, t4284, t5896, t5899, t5902, t656, t662, t97);
        let t21880 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1693(t21876, t655, t10201, t10202, t13448, t13451, t13453, t21818, t21821, t21824, t21827, t21830, t69);
        let t21881 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1694(t114, t21880);
        let (t21882, t21891, t21901, t21905, t21917) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1695(t30, t21881, t508, t1518, t5517, t13584, t9375, t6785, t9335, t3833, t5824, t18280, t2255, t513, t5549, t605, zeta_threshold);
        let (t21931, t21933) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1696(t33, t6792, t9350, t3841, t6416, t1113, t20256, t2255, t516, t5557, t162, t21917, t187, zeta_threshold);
        let (t21937, t21955, t21956) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1697(t30, t1450, t6922, t6785, t9605, t3874, t5824, t1344, t18280, t2255, t5574, t605, t6792, t9617, zeta_threshold);
        let t21969 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1698(t33, t3881, t6416, t1113, t1348, t20256, t21956, t2255, t5582, t21955, zeta_threshold);
    (t21876, t21881, t21882, t21891, t21901, t21905, t21931, t21933, t21937, t21969)
}
