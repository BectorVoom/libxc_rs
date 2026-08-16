//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta559 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1678;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1679;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1680;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1681;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1682;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta559(t324: f64, t88462: f64, t88475: f64, t41499: f64, t41502: f64, t88031: f64, t11409: f64, t11450: f64, t11509: f64, t15413: f64, t1621: f64, t1622: f64, t1634: f64, t23754: f64, t23755: f64, t23761: f64, t2943: f64, t2968: f64, t3012: f64, t3014: f64, t41759: f64, t4647: f64, t6157: f64, t6173: f64, t6177: f64, t6190: f64, t6205: f64, t63979: f64, t78111: f64, t78165: f64, t88008: f64, t88351: f64, t88368: f64, t88432: f64, t88445: f64, t88448: f64, t88451: f64, t2924: f64, t6110: f64, t6141: f64, t41908: f64, t63453: f64, t63459: f64, t63464: f64, t77499: f64, t77559: f64, t77561: f64, t88085: f64, t88089: f64, t88093: f64, t88097: f64, t51978: f64, t77505: f64, t77507: f64, t77509: f64, t88104: f64, t88108: f64, t88114: f64, t88118: f64, t88122: f64, t88126: f64, t88130: f64, t88134: f64, t15101: f64, t23767: f64, t15421: f64, t23770: f64, t11299: f64, t6145: f64, t11466: f64, t11507: f64, t15104: f64, t15350: f64, t15406: f64, t1633: f64, t23694: f64, t23723: f64, t23758: f64, t23764: f64, t23773: f64, t23785: f64, t2970: f64, t2987: f64, t311: f64, t52443: f64, t52812: f64, t6158: f64, t6189: f64, t6209: f64, t63997: f64, t64043: f64, t78207: f64, t88055: f64, t300: f64, t88077: f64, t88364: f64, t5023: f64, t63907: f64, t6400: f64, t88046: f64, t88048: f64, t88050: f64, t88052: f64, t88054: f64, t88140: f64, t88358: f64, t88361: f64, t88363: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t88477, t88481, t88499) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1678(t324, t88462, t88475, t41499, t41502, t88031, t11409, t11450, t11509, t15413, t1621, t1622, t1634, t23754, t23755, t23761, t2943, t2968, t3012, t3014, t41759, t4647, t6157, t6173, t6177, t6190, t6205, t63979, t78111, t78165, t88008, t88351, t88368, t88432, t88445, t88448, t88451);
        let (t88510, t88524) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1679(t2924, t6110, t6141, t41908, t63453, t63459, t63464, t77499, t77559, t77561, t88085, t88089, t88093, t88097);
        let t88537 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1680(t51978, t77505, t77507, t77509, t88104, t88108, t88114, t88118, t88122, t88126, t88130, t88134);
        let (t88562, t88564, t88567, t88570) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1681(t15101, t23767, t15421, t23770, t11299, t6141, t6145, t11450, t11466, t11507, t15104, t15350, t15406, t1633, t1634, t23694, t23723, t23758, t23764, t23773, t23785, t2968, t2970, t2987, t3012, t3014, t311, t52443, t52812, t6158, t6173, t6189, t6190, t6205, t6209, t63997, t64043, t78207, t88008, t88055, t88510, t88524, t88537);
        let (t88573, t88577) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1682(t300, t88077, t88364, t88499, t88570, t5023, t63907, t6400, t88046, t88048, t88050, t88052, t88054, t88140, t88358, t88361, t88363, t88368, t88432);
    (t88477, t88481, t88510, t88562, t88564, t88567, t88573, t88577)
}
