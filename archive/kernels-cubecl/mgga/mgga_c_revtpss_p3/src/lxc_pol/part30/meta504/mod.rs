//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta504 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1878;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1879;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1880;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1881;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1882;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1883;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta504<F: Float>(t1294: F, t7644: F, t7652: F, t1204: F, t2142: F, t1209: F, t26936: F, t1214: F, t7627: F, t7637: F, t1215: F, t1295: F, t2149: F, t2152: F, t26971: F, t26976: F, t26979: F, t26984: F, t26988: F, t26991: F, t26994: F, t26996: F, t26999: F, t27005: F, t27008: F, t27011: F, t3569: F, t3576: F, t3585: F, t3739: F, t7602: F, t7632: F, t7639: F, t7643: F, t7645: F, t7648: F, t7651: F, t7666: F, t265: F, t502: F, t26968: F, t3801: F, t7669: F, t12587: F, t2155: F, t1298: F, t1300: F, t198: F, t25743: F, t336: F, t3794: F, t3798: F, t5023: F, t7673: F, t33: F, t2159: F, t2258: F, t25791: F, t57: F, t606: F, t7677: F, t26816: F, t116: F, t7583: F, dens_threshold: F, rho1: F, zeta_threshold: F, t2371: F, t25812: F, t25814: F, t25816: F, t25818: F, t25820: F, t25834: F, t26800: F, t26804: F, t670: F, t7586: F, t118: F, t1310: F, t1453: F, t2163: F, t2320: F, t2322: F, t2328: F, t2331: F, t25085: F, t25092: F, t25095: F, t25180: F, t25182: F, t25184: F, t25186: F, t25189: F, t508: F, t569: F, t649: F, t7584: F, t7591: F, t7683: F, t7687: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t27015, t27020, t27025) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1878::<F>(t1294, t7644, t7652, t1204, t2142, t1209, t26936);
        let (t27029, t27032) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1879::<F>(t1214, t7627, t7637, t1215, t1295, t2149, t2152, t26971, t26976, t26979, t26984, t26988, t26991, t26994, t26996, t26999, t27005, t27008, t27011, t27015, t27020, t27025, t3569, t3576, t3585, t3739, t7602, t7632, t7639, t7643, t7645, t7648, t7651, t7666);
        let (t27033, t27037, t27041, t27048) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1880::<F>(t265, t502, t26968, t27032, t3801, t7669, t12587, t2155, t1298, t1300, t198, t25743, t336, t3794, t3798, t5023, t7673);
        let (t27056, t27060) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1881::<F>(t33, t2159, t2258, t25791, t27048, t57, t606, t7677, t26816, t116, t7583, dens_threshold, rho1, zeta_threshold);
        let t27066 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1882::<F>(t2371, t25812, t25814, t25816, t25818, t25820, t25834, t26800, t26804, t27060, t670, t7586);
        let t27075 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1883::<F>(t118, t1310, t1453, t2163, t2320, t2322, t2328, t2331, t25085, t25092, t25095, t25180, t25182, t25184, t25186, t25189, t26800, t26804, t27056, t27066, t508, t569, t649, t7584, t7586, t7591, t7683, t7687);
    (t27015, t27020, t27025, t27029, t27033, t27037, t27041, t27048, t27056, t27060, t27066, t27075)
}
