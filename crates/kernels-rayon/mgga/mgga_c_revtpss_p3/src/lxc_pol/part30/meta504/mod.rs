//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta504 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1878;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1879;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1880;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1881;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1882;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1883;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta504(t1294: f64, t7644: f64, t7652: f64, t1204: f64, t2142: f64, t1209: f64, t26936: f64, t1214: f64, t7627: f64, t7637: f64, t1215: f64, t1295: f64, t2149: f64, t2152: f64, t26971: f64, t26976: f64, t26979: f64, t26984: f64, t26988: f64, t26991: f64, t26994: f64, t26996: f64, t26999: f64, t27005: f64, t27008: f64, t27011: f64, t3569: f64, t3576: f64, t3585: f64, t3739: f64, t7602: f64, t7632: f64, t7639: f64, t7643: f64, t7645: f64, t7648: f64, t7651: f64, t7666: f64, t265: f64, t502: f64, t26968: f64, t3801: f64, t7669: f64, t12587: f64, t2155: f64, t1298: f64, t1300: f64, t198: f64, t25743: f64, t336: f64, t3794: f64, t3798: f64, t5023: f64, t7673: f64, t33: f64, t2159: f64, t2258: f64, t25791: f64, t57: f64, t606: f64, t7677: f64, t26816: f64, t116: f64, t7583: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t2371: f64, t25812: f64, t25814: f64, t25816: f64, t25818: f64, t25820: f64, t25834: f64, t26800: f64, t26804: f64, t670: f64, t7586: f64, t118: f64, t1310: f64, t1453: f64, t2163: f64, t2320: f64, t2322: f64, t2328: f64, t2331: f64, t25085: f64, t25092: f64, t25095: f64, t25180: f64, t25182: f64, t25184: f64, t25186: f64, t25189: f64, t508: f64, t569: f64, t649: f64, t7584: f64, t7591: f64, t7683: f64, t7687: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27015, t27020, t27025) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1878(t1294, t7644, t7652, t1204, t2142, t1209, t26936);
        let (t27029, t27032) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1879(t1214, t7627, t7637, t1215, t1295, t2149, t2152, t26971, t26976, t26979, t26984, t26988, t26991, t26994, t26996, t26999, t27005, t27008, t27011, t27015, t27020, t27025, t3569, t3576, t3585, t3739, t7602, t7632, t7639, t7643, t7645, t7648, t7651, t7666);
        let (t27033, t27037, t27041, t27048) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1880(t265, t502, t26968, t27032, t3801, t7669, t12587, t2155, t1298, t1300, t198, t25743, t336, t3794, t3798, t5023, t7673);
        let (t27056, t27060) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1881(t33, t2159, t2258, t25791, t27048, t57, t606, t7677, t26816, t116, t7583, dens_threshold, rho1, zeta_threshold);
        let t27066 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1882(t2371, t25812, t25814, t25816, t25818, t25820, t25834, t26800, t26804, t27060, t670, t7586);
        let t27075 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1883(t118, t1310, t1453, t2163, t2320, t2322, t2328, t2331, t25085, t25092, t25095, t25180, t25182, t25184, t25186, t25189, t26800, t26804, t27056, t27066, t508, t569, t649, t7584, t7586, t7591, t7683, t7687);
    (t27015, t27020, t27025, t27029, t27033, t27037, t27041, t27048, t27056, t27060, t27066, t27075)
}
