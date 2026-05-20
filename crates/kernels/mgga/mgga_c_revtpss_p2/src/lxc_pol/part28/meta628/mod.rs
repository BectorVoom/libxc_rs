//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2259;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2260;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2261;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2262;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2263;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2264;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta628<F: Float>(t5: F, t101152: F, t101185: F, t101225: F, t101259: F, t101309: F, t101340: F, t101371: F, t101402: F, t117: F, t2014: F, t25177: F, t7934: F, t28019: F, t531: F, t7238: F, t25866: F, t7898: F, t13867: F, t28167: F, t8996: F, t13872: F, t100940: F, t101120: F, t101124: F, t118: F, t1310: F, t14310: F, t1843: F, t2011: F, t25169: F, t25872: F, t28160: F, t4151: F, t4248: F, t508: F, t5517: F, t5787: F, t6983: F, t7231: F, t7894: F, t98615: F, t98617: F, t98621: F, t98623: F, t13517: F, t196: F, t197: F, t2035: F, t28196: F, t28197: F, t75365: F, t94976: F, t1513: F, t94975: F, t28036: F, t94978: F, t25823: F, t4287: F, t2340: F, t94982: F, t665: F, t25826: F, t2366: F, t13509: F, t6998: F, t94974: F, t94979: F, t94981: F, t114: F, t651: F, t530: F, t7933: F, t25865: F, t1353: F, t22496: F, t8717: F, t25082: F, t73394: F, t25188: F, t7937: F, t1936: F, t49686: F, t75667: F, t13426: F, t7002: F, t75485: F, t18227: F, t25832: F, t13514: F, t1518: F, t2371: F, t25805: F, t28025: F, t28030: F, t4292: F, t670: F, t6985: F, t92737: F, t97622: F, t97632: F, t98507: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t101407, t101416) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2259::<F>(t5, t101152, t101185, t101225, t101259, t101309, t101340, t101371, t101402, t117, t2014, t25177, t7934);
        let t101432 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2260::<F>(t28019, t531, t2014, t7238, t25866, t7898, t13867, t28167, t8996, t13872, t100940, t101120, t101124, t101407, t101416, t118, t1310, t14310, t1843, t2011, t25169, t25872, t28160, t4151, t4248, t508, t5517, t5787, t6983, t7231, t7894, t98615, t98617, t98621, t98623);
        let (t101436, t101439, t101448, t101451, t101453) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2261::<F>(t13517, t196, t197, t2035, t28196, t28197, t75365, t94976, t1513, t94975, t28036, t94978);
        let t101468 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2262::<F>(t101453, t25823, t4287, t1513, t2340, t94982, t665, t25826, t2366, t13509, t6998, t101448, t101451, t94974, t94979, t94981);
        let (t101469, t101472, t101476, t101482, t101485) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2263::<F>(t114, t101468, t508, t651, t530, t7933, t2014, t25865, t1353, t22496, t28167, t8717, t25082, t73394);
        let (t101486, t101515) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2264::<F>(t25188, t7937, t1936, t49686, t75667, t13426, t7002, t75485, t18227, t25832, t4248, t13514, t1518, t2371, t25805, t28025, t28030, t4292, t670, t6985, t92737, t97622, t97632, t98507);
    (t101407, t101432, t101436, t101439, t101469, t101472, t101476, t101482, t101485, t101486, t101515)
}
