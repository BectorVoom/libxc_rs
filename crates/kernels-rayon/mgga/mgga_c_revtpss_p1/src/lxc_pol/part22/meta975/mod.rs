//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta975 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3276;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3277;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3278;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3279;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3280;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3281;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3282;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3283;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta975(t10777: f64, t14671: f64, t14686: f64, t4424: f64, t61956: f64, t837: f64, t18477: f64, t50769: f64, t51133: f64, t18348: f64, t2710: f64, t2713: f64, t2394: f64, t40862: f64, t40868: f64, t51110: f64, t51112: f64, t51121: f64, t51125: f64, t51135: f64, t5988: f64, t800: f64, t39419: f64, t39422: f64, t39429: f64, t39432: f64, t39442: f64, t61019: f64, t61021: f64, t61022: f64, t61026: f64, t61027: f64, t61028: f64, t61029: f64, t61031: f64, t61032: f64, t61039: f64, t61088: f64, t61091: f64, t61094: f64, t61097: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t39534: f64, t39537: f64, t39540: f64, t39741: f64, t39744: f64, t39747: f64, t39750: f64, t39756: f64, t61101: f64, t61115: f64, t61116: f64, t61124: f64, t61131: f64, t61135: f64, t61138: f64, t39760: f64, t39764: f64, t39770: f64, t39773: f64, t39779: f64, t39783: f64, t39786: f64, t39791: f64, t39795: f64, t61149: f64, t61150: f64, t61151: f64, t61159: f64, t61161: f64, t61162: f64, t61166: f64, t61167: f64, t61168: f64, t61169: f64, t39799: f64, t39807: f64, t39813: f64, t39818: f64, t39823: f64, t40084: f64, t40088: f64, t40099: f64, t61170: f64, t61171: f64, t61172: f64, t61173: f64, t61177: f64, t61179: f64, t61181: f64, t61190: f64, t61191: f64, t61197: f64, t61198: f64, t61199: f64, t40103: f64, t40115: f64, t40131: f64, t40137: f64, t61200: f64, t61202: f64, t61209: f64, t61214: f64, t61215: f64, t61219: f64, t61220: f64, t61222: f64, t61224: f64, t61225: f64, t61229: f64, t61240: f64, t61244: f64, t61245: f64, t61248: f64, t39989: f64, t61249: f64, t61250: f64, t61261: f64, t61265: f64, t61269: f64, t61274: f64, t61283: f64, t61286: f64, t61287: f64, t61288: f64, t61290: f64, t61292: f64, t61293: f64, t61295: f64, t61297: f64, t61300: f64, t61302: f64, t61306: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t62236, t62241, t62246, t62251) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3276(t10777, t14671, t14686, t4424, t61956, t837, t18477, t50769, t51133, t18348, t2710, t2713);
        let t62258 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3277(t2394, t40862, t40868, t51110, t51112, t51121, t51125, t51135, t5988, t62236, t62241, t62246, t62251, t800);
        let t62259 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3278(t39419, t39422, t39429, t39432, t39442, t61019, t61021, t61022, t61026, t61027, t61028, t61029, t61031, t61032, t61039, t61088, t61091, t61094, t61097);
        let t62260 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3279(t39483, t39520, t39528, t39531, t39534, t39537, t39540, t39741, t39744, t39747, t39750, t39756, t61101, t61115, t61116, t61124, t61131, t61135, t61138);
        let t62262 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3280(t39760, t39764, t39770, t39773, t39779, t39783, t39786, t39791, t39795, t61149, t61150, t61151, t61159, t61161, t61162, t61166, t61167, t61168, t61169);
        let t62263 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3281(t39799, t39807, t39813, t39818, t39823, t40084, t40088, t40099, t61170, t61171, t61172, t61173, t61177, t61179, t61181, t61190, t61191, t61197, t61198, t61199);
        let t62266 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3282(t40103, t40115, t40131, t40137, t61200, t61202, t61209, t61214, t61215, t61219, t61220, t61222, t61224, t61225, t61229, t61240, t61244, t61245, t61248);
        let t62267 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3283(t39989, t61249, t61250, t61261, t61265, t61269, t61274, t61283, t61286, t61287, t61288, t61290, t61292, t61293, t61295, t61297, t61300, t61302, t61306);
    (t62258, t62259, t62260, t62262, t62263, t62266, t62267)
}
