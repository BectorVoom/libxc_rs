//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta493 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1760;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1761;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1762;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1763;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1764;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1765;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1766;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta493(t2047: f64, t28089: f64, t7349: f64, t7702: f64, t7348: f64, t7719: f64, t1923: f64, t2048: f64, t25162: f64, t26170: f64, t26175: f64, t26182: f64, t26190: f64, t26207: f64, t28093: f64, t28133: f64, t28147: f64, t28154: f64, t28628: f64, t6954: f64, t6963: f64, t7343: f64, t7352: f64, t7964: f64, t5: f64, t28621: f64, t117: f64, t116: f64, t7968: f64, t2051: f64, t670: f64, t114: f64, t28034: f64, t25825: f64, t26148: f64, t28037: f64, t28039: f64, t1312: f64, t13426: f64, t1518: f64, t18227: f64, t2055: f64, t2322: f64, t26399: f64, t27123: f64, t28219: f64, t4248: f64, t4292: f64, t5523: f64, t7359: f64, t7373: f64, t7889: f64, t7983: f64, t1843: f64, t118: f64, t1502: f64, t1911: f64, t2052: f64, t2056: f64, t2089: f64, t25082: f64, t28196: f64, t28287: f64, t28586: f64, t28588: f64, t4246: f64, t5517: f64, t569: f64, t651: f64, t671: f64, t7357: f64, t7367: f64, t7474: f64, t7484: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28635, t28638, t28640) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1760(t2047, t28089, t7349, t7702, t7348, t7719);
        let (t28641, t28649) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1761(t1923, t28640, t2048, t25162, t26170, t26175, t26182, t26190, t26207, t28093, t28133, t28147, t28154, t28628, t28635, t28638, t6954, t6963, t7343, t7352, t7702, t7964);
        let (t28651, t28652, t28653) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1762(t5, t28621, t28649, t117, t116, t7968);
        let t28658 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1763(t2051, t670);
        let t28683 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1764(t114, t28034, t25825, t26148, t28037, t28039);
        let t28686 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1765(t1312, t13426, t1518, t18227, t2055, t2322, t26399, t27123, t28219, t28652, t28653, t28658, t28683, t4248, t4292, t5523, t670, t7359, t7373, t7889, t7983);
        let (t28696, t28699) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1766(t1843, t7373, t118, t13426, t1502, t18227, t1911, t2052, t2056, t2089, t25082, t28196, t28287, t28586, t28588, t28653, t28686, t4246, t4248, t5517, t569, t651, t671, t7357, t7367, t7474, t7484);
    (t28635, t28638, t28640, t28641, t28651, t28652, t28653, t28658, t28683, t28686, t28696, t28699)
}
