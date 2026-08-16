//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta902 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2879;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2880;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2881;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2882;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2883;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2884;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2885;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2886;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2887;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2888;
use chunk10::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2889;
use chunk11::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2890;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta902(t1583: f64, t1940: f64, t39783: f64, t39786: f64, t39791: f64, t39795: f64, t39799: f64, t49958: f64, t49964: f64, t49982: f64, t63160: f64, t76974: f64, t76976: f64, t39807: f64, t39813: f64, t39818: f64, t39823: f64, t40084: f64, t40088: f64, t76977: f64, t76978: f64, t76980: f64, t76986: f64, t76987: f64, t1544: f64, t23111: f64, t23148: f64, t2403: f64, t2404: f64, t40131: f64, t40137: f64, t50080: f64, t61139: f64, t76999: f64, t77000: f64, t77002: f64, t77003: f64, t77004: f64, t77005: f64, t5966: f64, t890: f64, t18435: f64, t18498: f64, t39989: f64, t40150: f64, t4541: f64, t4546: f64, t4556: f64, t50098: f64, t77007: f64, t77008: f64, t77009: f64, t77010: f64, t77011: f64, t5962: f64, t18850: f64, t18860: f64, t18865: f64, t27375: f64, t4343: f64, t4433: f64, t50866: f64, t63146: f64, t77012: f64, t77013: f64, t77014: f64, t77015: f64, t77020: f64, t4537: f64, t14353: f64, t18268: f64, t40167: f64, t40171: f64, t40184: f64, t50884: f64, t77024: f64, t77025: f64, t77026: f64, t77027: f64, t77028: f64, t23421: f64, t892: f64, t18871: f64, t18875: f64, t77029: f64, t77032: f64, t77036: f64, t77038: f64, t77039: f64, t77040: f64, t77041: f64, t77045: f64, t775: f64, t40076: f64, t40079: f64, t40194: f64, t40198: f64, t77048: f64, t77051: f64, t77053: f64, t77056: f64, t77058: f64, t77059: f64, t77060: f64, t18392: f64, t198: f64, t207: f64, t23114: f64, t23279: f64, t27384: f64, t39419: f64, t39422: f64, t39483: f64, t39741: f64, t39744: f64, t39747: f64, t39750: f64, t40067: f64, t40072: f64, t40099: f64, t40103: f64, t40115: f64, t50048: f64, t50874: f64, t61033: f64, t75970: f64, t75990: f64, t76012: f64, t76038: f64, t76055: f64, t76077: f64, t76421: f64, t765: f64, t76890: f64, t76893: f64, t76932: f64, t76935: f64, t76936: f64, t76942: f64, t76944: f64, t76946: f64, t76948: f64, t76950: f64, t76951: f64, t76952: f64, t76954: f64, t76963: f64, t76966: f64, t76988: f64, t76991: f64, t76992: f64, t76995: f64, t76997: f64, t76998: f64, t77021: f64, t77023: f64, t77298: f64, t77326: f64, t77333: f64, t77347: f64, t77360: f64, t77373: f64, t77381: f64, t2: f64, t580: f64, t6084: f64, t19049: f64, t4729: f64, t23649: f64, t3022: f64, t19023: f64, t4719: f64, t23457: f64, t23478: f64, t689: f64, t76397: f64, t905: f64, t128: f64, t904: f64, t23489: f64, t23482: f64, t23486: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t77386 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2879(t1583, t1940, t39783, t39786, t39791, t39795, t39799, t49958, t49964, t49982, t63160, t76974, t76976);
        let (t77387, t77400) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2880(t39807, t39813, t39818, t39823, t40084, t40088, t76977, t76978, t76980, t76986, t76987, t1544, t23111, t23148, t2403, t2404, t40131, t40137, t50080, t61139, t76999, t77000, t77002, t77003, t77004, t77005);
        let t77412 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2881(t5966, t890, t18435, t18498, t39989, t40150, t4541, t4546, t4556, t50098, t77007, t77008, t77009, t77010, t77011);
        let t77429 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2882(t5962, t890, t1544, t18850, t18860, t18865, t2403, t27375, t4343, t4433, t4541, t4556, t50866, t63146, t77012, t77013, t77014, t77015, t77020);
        let (t77441, t77455) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2883(t1544, t4537, t14353, t18268, t2403, t40167, t40171, t40184, t4433, t4541, t50884, t5962, t77024, t77025, t77026, t77027, t77028);
        let t77467 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2884(t23421, t892, t18865, t18871, t18875, t2403, t77029, t77032, t77036, t77038, t77039, t77040, t77041, t77045, t775);
        let t77472 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2885(t40076, t40079, t40194, t40198, t77048, t77051, t77053, t77056, t77058, t77059, t77060, t14353, t18392, t18850, t18865, t18871, t1940, t198, t207, t23114, t23279, t2403, t2404, t27384, t39419, t39422, t39483, t39741, t39744, t39747, t39750, t40067, t40072, t40099, t40103, t40115, t4343, t4537, t4541, t4546, t4556, t50048, t50874, t5966, t61033, t75970, t75990, t76012, t76038, t76055, t76077, t76421, t765, t76890, t76893, t76932, t76935, t76936, t76942, t76944, t76946, t76948, t76950, t76951, t76952, t76954, t76963, t76966, t76988, t76991, t76992, t76995, t76997, t76998, t77021, t77023, t77298, t77326, t77333, t77347, t77360, t77373, t77381, t77386, t77387, t77400, t77412, t77429, t77441, t77455, t77467, t775, t890, t892);
        let (t77481, t77492, t77494, t77496, t77498, t77499) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2886(t2, t580, t6084, t19049, t4729, t23649, t3022, t19023, t4719, t23457, t23478, t689);
        let (t77501, t77503) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2887(t76397, t905, t128, t904);
        let t77505 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2888(t23489, t689);
        let t77507 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2889(t23482, t689);
        let t77509 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2890(t23486, t689);
    (t77472, t77481, t77492, t77494, t77496, t77498, t77499, t77501, t77503, t77505, t77507, t77509)
}
