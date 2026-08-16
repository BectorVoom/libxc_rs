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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3276;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3277;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3278;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3279;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3280;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3281;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3282;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3283;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta975<F: Float>(t10777: F, t14671: F, t14686: F, t4424: F, t61956: F, t837: F, t18477: F, t50769: F, t51133: F, t18348: F, t2710: F, t2713: F, t2394: F, t40862: F, t40868: F, t51110: F, t51112: F, t51121: F, t51125: F, t51135: F, t5988: F, t800: F, t39419: F, t39422: F, t39429: F, t39432: F, t39442: F, t61019: F, t61021: F, t61022: F, t61026: F, t61027: F, t61028: F, t61029: F, t61031: F, t61032: F, t61039: F, t61088: F, t61091: F, t61094: F, t61097: F, t39483: F, t39520: F, t39528: F, t39531: F, t39534: F, t39537: F, t39540: F, t39741: F, t39744: F, t39747: F, t39750: F, t39756: F, t61101: F, t61115: F, t61116: F, t61124: F, t61131: F, t61135: F, t61138: F, t39760: F, t39764: F, t39770: F, t39773: F, t39779: F, t39783: F, t39786: F, t39791: F, t39795: F, t61149: F, t61150: F, t61151: F, t61159: F, t61161: F, t61162: F, t61166: F, t61167: F, t61168: F, t61169: F, t39799: F, t39807: F, t39813: F, t39818: F, t39823: F, t40084: F, t40088: F, t40099: F, t61170: F, t61171: F, t61172: F, t61173: F, t61177: F, t61179: F, t61181: F, t61190: F, t61191: F, t61197: F, t61198: F, t61199: F, t40103: F, t40115: F, t40131: F, t40137: F, t61200: F, t61202: F, t61209: F, t61214: F, t61215: F, t61219: F, t61220: F, t61222: F, t61224: F, t61225: F, t61229: F, t61240: F, t61244: F, t61245: F, t61248: F, t39989: F, t61249: F, t61250: F, t61261: F, t61265: F, t61269: F, t61274: F, t61283: F, t61286: F, t61287: F, t61288: F, t61290: F, t61292: F, t61293: F, t61295: F, t61297: F, t61300: F, t61302: F, t61306: F) -> (F, F, F, F, F, F, F) {
        let (t62236, t62241, t62246, t62251) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3276::<F>(t10777, t14671, t14686, t4424, t61956, t837, t18477, t50769, t51133, t18348, t2710, t2713);
        let t62258 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3277::<F>(t2394, t40862, t40868, t51110, t51112, t51121, t51125, t51135, t5988, t62236, t62241, t62246, t62251, t800);
        let t62259 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3278::<F>(t39419, t39422, t39429, t39432, t39442, t61019, t61021, t61022, t61026, t61027, t61028, t61029, t61031, t61032, t61039, t61088, t61091, t61094, t61097);
        let t62260 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3279::<F>(t39483, t39520, t39528, t39531, t39534, t39537, t39540, t39741, t39744, t39747, t39750, t39756, t61101, t61115, t61116, t61124, t61131, t61135, t61138);
        let t62262 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3280::<F>(t39760, t39764, t39770, t39773, t39779, t39783, t39786, t39791, t39795, t61149, t61150, t61151, t61159, t61161, t61162, t61166, t61167, t61168, t61169);
        let t62263 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3281::<F>(t39799, t39807, t39813, t39818, t39823, t40084, t40088, t40099, t61170, t61171, t61172, t61173, t61177, t61179, t61181, t61190, t61191, t61197, t61198, t61199);
        let t62266 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3282::<F>(t40103, t40115, t40131, t40137, t61200, t61202, t61209, t61214, t61215, t61219, t61220, t61222, t61224, t61225, t61229, t61240, t61244, t61245, t61248);
        let t62267 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3283::<F>(t39989, t61249, t61250, t61261, t61265, t61269, t61274, t61283, t61286, t61287, t61288, t61290, t61292, t61293, t61295, t61297, t61300, t61302, t61306);
    (t62258, t62259, t62260, t62262, t62263, t62266, t62267)
}
