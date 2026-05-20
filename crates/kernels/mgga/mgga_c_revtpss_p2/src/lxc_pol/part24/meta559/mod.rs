//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta559 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1678;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1679;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1680;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1681;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1682;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta559<F: Float>(t324: F, t88462: F, t88475: F, t41499: F, t41502: F, t88031: F, t11409: F, t11450: F, t11509: F, t15413: F, t1621: F, t1622: F, t1634: F, t23754: F, t23755: F, t23761: F, t2943: F, t2968: F, t3012: F, t3014: F, t41759: F, t4647: F, t6157: F, t6173: F, t6177: F, t6190: F, t6205: F, t63979: F, t78111: F, t78165: F, t88008: F, t88351: F, t88368: F, t88432: F, t88445: F, t88448: F, t88451: F, t2924: F, t6110: F, t6141: F, t41908: F, t63453: F, t63459: F, t63464: F, t77499: F, t77559: F, t77561: F, t88085: F, t88089: F, t88093: F, t88097: F, t51978: F, t77505: F, t77507: F, t77509: F, t88104: F, t88108: F, t88114: F, t88118: F, t88122: F, t88126: F, t88130: F, t88134: F, t15101: F, t23767: F, t15421: F, t23770: F, t11299: F, t6145: F, t11466: F, t11507: F, t15104: F, t15350: F, t15406: F, t1633: F, t23694: F, t23723: F, t23758: F, t23764: F, t23773: F, t23785: F, t2970: F, t2987: F, t311: F, t52443: F, t52812: F, t6158: F, t6189: F, t6209: F, t63997: F, t64043: F, t78207: F, t88055: F, t300: F, t88077: F, t88364: F, t5023: F, t63907: F, t6400: F, t88046: F, t88048: F, t88050: F, t88052: F, t88054: F, t88140: F, t88358: F, t88361: F, t88363: F) -> (F, F, F, F, F, F, F, F) {
        let (t88477, t88481, t88499) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1678::<F>(t324, t88462, t88475, t41499, t41502, t88031, t11409, t11450, t11509, t15413, t1621, t1622, t1634, t23754, t23755, t23761, t2943, t2968, t3012, t3014, t41759, t4647, t6157, t6173, t6177, t6190, t6205, t63979, t78111, t78165, t88008, t88351, t88368, t88432, t88445, t88448, t88451);
        let (t88510, t88524) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1679::<F>(t2924, t6110, t6141, t41908, t63453, t63459, t63464, t77499, t77559, t77561, t88085, t88089, t88093, t88097);
        let t88537 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1680::<F>(t51978, t77505, t77507, t77509, t88104, t88108, t88114, t88118, t88122, t88126, t88130, t88134);
        let (t88562, t88564, t88567, t88570) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1681::<F>(t15101, t23767, t15421, t23770, t11299, t6141, t6145, t11450, t11466, t11507, t15104, t15350, t15406, t1633, t1634, t23694, t23723, t23758, t23764, t23773, t23785, t2968, t2970, t2987, t3012, t3014, t311, t52443, t52812, t6158, t6173, t6189, t6190, t6205, t6209, t63997, t64043, t78207, t88008, t88055, t88510, t88524, t88537);
        let (t88573, t88577) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1682::<F>(t300, t88077, t88364, t88499, t88570, t5023, t63907, t6400, t88046, t88048, t88050, t88052, t88054, t88140, t88358, t88361, t88363, t88368, t88432);
    (t88477, t88481, t88510, t88562, t88564, t88567, t88573, t88577)
}
