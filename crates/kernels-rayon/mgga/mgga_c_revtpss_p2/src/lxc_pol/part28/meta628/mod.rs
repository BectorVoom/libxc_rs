//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2259;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2260;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2261;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2262;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2263;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2264;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta628(t5: f64, t101152: f64, t101185: f64, t101225: f64, t101259: f64, t101309: f64, t101340: f64, t101371: f64, t101402: f64, t117: f64, t2014: f64, t25177: f64, t7934: f64, t28019: f64, t531: f64, t7238: f64, t25866: f64, t7898: f64, t13867: f64, t28167: f64, t8996: f64, t13872: f64, t100940: f64, t101120: f64, t101124: f64, t118: f64, t1310: f64, t14310: f64, t1843: f64, t2011: f64, t25169: f64, t25872: f64, t28160: f64, t4151: f64, t4248: f64, t508: f64, t5517: f64, t5787: f64, t6983: f64, t7231: f64, t7894: f64, t98615: f64, t98617: f64, t98621: f64, t98623: f64, t13517: f64, t196: f64, t197: f64, t2035: f64, t28196: f64, t28197: f64, t75365: f64, t94976: f64, t1513: f64, t94975: f64, t28036: f64, t94978: f64, t25823: f64, t4287: f64, t2340: f64, t94982: f64, t665: f64, t25826: f64, t2366: f64, t13509: f64, t6998: f64, t94974: f64, t94979: f64, t94981: f64, t114: f64, t651: f64, t530: f64, t7933: f64, t25865: f64, t1353: f64, t22496: f64, t8717: f64, t25082: f64, t73394: f64, t25188: f64, t7937: f64, t1936: f64, t49686: f64, t75667: f64, t13426: f64, t7002: f64, t75485: f64, t18227: f64, t25832: f64, t13514: f64, t1518: f64, t2371: f64, t25805: f64, t28025: f64, t28030: f64, t4292: f64, t670: f64, t6985: f64, t92737: f64, t97622: f64, t97632: f64, t98507: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t101407, t101416) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2259(t5, t101152, t101185, t101225, t101259, t101309, t101340, t101371, t101402, t117, t2014, t25177, t7934);
        let t101432 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2260(t28019, t531, t2014, t7238, t25866, t7898, t13867, t28167, t8996, t13872, t100940, t101120, t101124, t101407, t101416, t118, t1310, t14310, t1843, t2011, t25169, t25872, t28160, t4151, t4248, t508, t5517, t5787, t6983, t7231, t7894, t98615, t98617, t98621, t98623);
        let (t101436, t101439, t101448, t101451, t101453) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2261(t13517, t196, t197, t2035, t28196, t28197, t75365, t94976, t1513, t94975, t28036, t94978);
        let t101468 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2262(t101453, t25823, t4287, t1513, t2340, t94982, t665, t25826, t2366, t13509, t6998, t101448, t101451, t94974, t94979, t94981);
        let (t101469, t101472, t101476, t101482, t101485) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2263(t114, t101468, t508, t651, t530, t7933, t2014, t25865, t1353, t22496, t28167, t8717, t25082, t73394);
        let (t101486, t101515) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2264(t25188, t7937, t1936, t49686, t75667, t13426, t7002, t75485, t18227, t25832, t4248, t13514, t1518, t2371, t25805, t28025, t28030, t4292, t670, t6985, t92737, t97622, t97632, t98507);
    (t101407, t101432, t101436, t101439, t101469, t101472, t101476, t101482, t101485, t101486, t101515)
}
