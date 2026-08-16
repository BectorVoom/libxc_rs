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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta348(t1413: f64, t5591: f64, t547: f64, t807: f64, t5609: f64, t808: f64, t9845: f64, t1885: f64, t9909: f64, t13944: f64, t3936: f64, t3938: f64, t13937: f64, t13943: f64, t13946: f64, t13949: f64, t3934: f64, t9796: f64, t9799: f64, t9804: f64, t9822: f64, t5674: f64, t9810: f64, t125: f64, t1399: f64, t4057: f64, t5704: f64, t1872: f64, t9818: f64, t9816: f64, t5706: f64, t9962: f64, t5673: f64, t5675: f64, t9955: f64, t9956: f64, t4000: f64, t820: f64, t844: f64, t5677: f64, t5671: f64, t9847: f64, t9896: f64, t9906: f64, t9910: f64, t9919: f64, t13847: f64, t13848: f64, t2713: f64, t3964: f64, t5617: f64, t3829: f64, t800: f64, t124: f64, t13716: f64, t5686: f64, t9744: f64, t1353: f64, t5689: f64, t3889: f64, t1370: f64, t3944: f64, t9748: f64, t9924: f64, t9926: f64, t9932: f64, t9937: f64, t9953: f64, t221: f64, t4019: f64, t5659: f64, t4018: f64, t3989: f64, t5629: f64, t3930: f64, t5661: f64, t5665: f64, t9976: f64, t1412: f64, t1882: f64, t3992: f64, t2661: f64, t5608: f64, t5651: f64, t10004: f64, t9963: f64, t9971: f64, t9973: f64, t9977: f64, t9982: f64, t13773: f64, t13814: f64, t13860: f64, t13931: f64, t225: f64, t5774: f64, t72: f64, t686: f64, t3915: f64, t5711: f64, t786: f64, t1364: f64, t1357: f64, t5775: f64, t689: f64, t213: f64, t4071: f64, t561: f64, t5728: f64, t9666: f64, t9668: f64, t9672: f64, t9677: f64, t9683: f64, t9687: f64, t9691: f64, t9694: f64, t2470: f64, t5721: f64, t1445: f64, t5599: f64, t2435: f64, t5600: f64, t1426: f64, t1893: f64, t3917: f64, t136: f64, t1903: f64, t2457: f64, t9674: f64, t10175: f64, t5722: f64, t122: f64, t3916: f64, t9680: f64, t1437: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13954, t13956, t13959, t13962) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1177(t1413, t5591, t547, t807, t5609, t808, t9845, t1885, t9909, t13944, t3936, t3938);
        let t13965 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1178(t13937, t13943, t13946, t13949, t13954, t13956, t13959, t13962, t3934, t9796, t9799, t9804, t9822);
        let (t13967, t13977, t13981, t13987, t13988) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1179(t3936, t5674, t9810, t125, t5591, t1399, t4057, t5704, t1872, t9818, t9816, t5706, t9962);
        let t14002 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1180(t13944, t5673, t5675, t5674, t9955, t9956, t4000, t820, t844, t5677, t13967, t13977, t13981, t13987, t13988, t3934, t5671, t9847, t9896, t9906, t9910, t9919);
        let (t14007, t14013, t14016, t14019) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1181(t13847, t13848, t1399, t9816, t2713, t3964, t5617, t1872, t3829, t800, t124, t13716);
        let t14033 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1182(t14019, t800, t5686, t9744, t1353, t5689, t1872, t3889, t1370, t14007, t14013, t14016, t3944, t9748, t9924, t9926, t9932, t9937, t9953);
        let (t14038, t14040, t14042, t14043, t14045) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1183(t221, t4019, t5659, t4018, t3989, t5629, t3930, t5661, t5665, t9976, t1412, t1882);
        let t14063 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1184(t14045, t3938, t3992, t2661, t1399, t5608, t5651, t10004, t14038, t14040, t14042, t14043, t9963, t9971, t9973, t9977, t9982);
        let (t14066, t14067, t14079) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1185(t13773, t13814, t13860, t13931, t13965, t14002, t14033, t14063, t225, t5774, t72, t686);
        let t14088 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1186(t14079, t3915, t5711, t786, t1364, t1357, t5775, t689, t14067, t213, t4071, t561, t5728, t9666, t9668, t9672, t9677, t9683, t9687, t9691, t9694);
        let (t14091, t14096, t14097, t14102) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1187(t2470, t5721, t3915, t1445, t5599, t689, t2435, t5600, t1426, t1893, t786, t3917);
        let (t14105, t14108, t14111, t14113) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1188(t136, t1903, t2457, t9674, t10175, t5722, t122, t5721, t3916, t9680, t1437, t1882);
    (t14066, t14088, t14091, t14096, t14097, t14102, t14105, t14108, t14111, t14113)
}
