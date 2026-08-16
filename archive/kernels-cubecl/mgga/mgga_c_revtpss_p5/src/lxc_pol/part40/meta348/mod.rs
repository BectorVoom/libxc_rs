//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta348 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1177;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1178;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1179;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1180;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1181;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1182;
use chunk6::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1183;
use chunk7::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1184;
use chunk8::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1185;
use chunk9::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1186;
use chunk10::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1187;
use chunk11::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1188;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta348<F: Float>(t1413: F, t5591: F, t547: F, t807: F, t5609: F, t808: F, t9845: F, t1885: F, t9909: F, t13944: F, t3936: F, t3938: F, t13937: F, t13943: F, t13946: F, t13949: F, t3934: F, t9796: F, t9799: F, t9804: F, t9822: F, t5674: F, t9810: F, t125: F, t1399: F, t4057: F, t5704: F, t1872: F, t9818: F, t9816: F, t5706: F, t9962: F, t5673: F, t5675: F, t9955: F, t9956: F, t4000: F, t820: F, t844: F, t5677: F, t5671: F, t9847: F, t9896: F, t9906: F, t9910: F, t9919: F, t13847: F, t13848: F, t2713: F, t3964: F, t5617: F, t3829: F, t800: F, t124: F, t13716: F, t5686: F, t9744: F, t1353: F, t5689: F, t3889: F, t1370: F, t3944: F, t9748: F, t9924: F, t9926: F, t9932: F, t9937: F, t9953: F, t221: F, t4019: F, t5659: F, t4018: F, t3989: F, t5629: F, t3930: F, t5661: F, t5665: F, t9976: F, t1412: F, t1882: F, t3992: F, t2661: F, t5608: F, t5651: F, t10004: F, t9963: F, t9971: F, t9973: F, t9977: F, t9982: F, t13773: F, t13814: F, t13860: F, t13931: F, t225: F, t5774: F, t72: F, t686: F, t3915: F, t5711: F, t786: F, t1364: F, t1357: F, t5775: F, t689: F, t213: F, t4071: F, t561: F, t5728: F, t9666: F, t9668: F, t9672: F, t9677: F, t9683: F, t9687: F, t9691: F, t9694: F, t2470: F, t5721: F, t1445: F, t5599: F, t2435: F, t5600: F, t1426: F, t1893: F, t3917: F, t136: F, t1903: F, t2457: F, t9674: F, t10175: F, t5722: F, t122: F, t3916: F, t9680: F, t1437: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13954, t13956, t13959, t13962) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1177::<F>(t1413, t5591, t547, t807, t5609, t808, t9845, t1885, t9909, t13944, t3936, t3938);
        let t13965 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1178::<F>(t13937, t13943, t13946, t13949, t13954, t13956, t13959, t13962, t3934, t9796, t9799, t9804, t9822);
        let (t13967, t13977, t13981, t13987, t13988) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1179::<F>(t3936, t5674, t9810, t125, t5591, t1399, t4057, t5704, t1872, t9818, t9816, t5706, t9962);
        let t14002 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1180::<F>(t13944, t5673, t5675, t5674, t9955, t9956, t4000, t820, t844, t5677, t13967, t13977, t13981, t13987, t13988, t3934, t5671, t9847, t9896, t9906, t9910, t9919);
        let (t14007, t14013, t14016, t14019) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1181::<F>(t13847, t13848, t1399, t9816, t2713, t3964, t5617, t1872, t3829, t800, t124, t13716);
        let t14033 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1182::<F>(t14019, t800, t5686, t9744, t1353, t5689, t1872, t3889, t1370, t14007, t14013, t14016, t3944, t9748, t9924, t9926, t9932, t9937, t9953);
        let (t14038, t14040, t14042, t14043, t14045) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1183::<F>(t221, t4019, t5659, t4018, t3989, t5629, t3930, t5661, t5665, t9976, t1412, t1882);
        let t14063 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1184::<F>(t14045, t3938, t3992, t2661, t1399, t5608, t5651, t10004, t14038, t14040, t14042, t14043, t9963, t9971, t9973, t9977, t9982);
        let (t14066, t14067, t14079) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1185::<F>(t13773, t13814, t13860, t13931, t13965, t14002, t14033, t14063, t225, t5774, t72, t686);
        let t14088 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1186::<F>(t14079, t3915, t5711, t786, t1364, t1357, t5775, t689, t14067, t213, t4071, t561, t5728, t9666, t9668, t9672, t9677, t9683, t9687, t9691, t9694);
        let (t14091, t14096, t14097, t14102) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1187::<F>(t2470, t5721, t3915, t1445, t5599, t689, t2435, t5600, t1426, t1893, t786, t3917);
        let (t14105, t14108, t14111, t14113) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1188::<F>(t136, t1903, t2457, t9674, t10175, t5722, t122, t5721, t3916, t9680, t1437, t1882);
    (t14066, t14088, t14091, t14096, t14097, t14102, t14105, t14108, t14111, t14113)
}
