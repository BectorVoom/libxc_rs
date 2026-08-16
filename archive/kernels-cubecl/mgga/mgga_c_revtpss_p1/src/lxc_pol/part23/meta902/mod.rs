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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta902<F: Float>(t1583: F, t1940: F, t39783: F, t39786: F, t39791: F, t39795: F, t39799: F, t49958: F, t49964: F, t49982: F, t63160: F, t76974: F, t76976: F, t39807: F, t39813: F, t39818: F, t39823: F, t40084: F, t40088: F, t76977: F, t76978: F, t76980: F, t76986: F, t76987: F, t1544: F, t23111: F, t23148: F, t2403: F, t2404: F, t40131: F, t40137: F, t50080: F, t61139: F, t76999: F, t77000: F, t77002: F, t77003: F, t77004: F, t77005: F, t5966: F, t890: F, t18435: F, t18498: F, t39989: F, t40150: F, t4541: F, t4546: F, t4556: F, t50098: F, t77007: F, t77008: F, t77009: F, t77010: F, t77011: F, t5962: F, t18850: F, t18860: F, t18865: F, t27375: F, t4343: F, t4433: F, t50866: F, t63146: F, t77012: F, t77013: F, t77014: F, t77015: F, t77020: F, t4537: F, t14353: F, t18268: F, t40167: F, t40171: F, t40184: F, t50884: F, t77024: F, t77025: F, t77026: F, t77027: F, t77028: F, t23421: F, t892: F, t18871: F, t18875: F, t77029: F, t77032: F, t77036: F, t77038: F, t77039: F, t77040: F, t77041: F, t77045: F, t775: F, t40076: F, t40079: F, t40194: F, t40198: F, t77048: F, t77051: F, t77053: F, t77056: F, t77058: F, t77059: F, t77060: F, t18392: F, t198: F, t207: F, t23114: F, t23279: F, t27384: F, t39419: F, t39422: F, t39483: F, t39741: F, t39744: F, t39747: F, t39750: F, t40067: F, t40072: F, t40099: F, t40103: F, t40115: F, t50048: F, t50874: F, t61033: F, t75970: F, t75990: F, t76012: F, t76038: F, t76055: F, t76077: F, t76421: F, t765: F, t76890: F, t76893: F, t76932: F, t76935: F, t76936: F, t76942: F, t76944: F, t76946: F, t76948: F, t76950: F, t76951: F, t76952: F, t76954: F, t76963: F, t76966: F, t76988: F, t76991: F, t76992: F, t76995: F, t76997: F, t76998: F, t77021: F, t77023: F, t77298: F, t77326: F, t77333: F, t77347: F, t77360: F, t77373: F, t77381: F, t2: F, t580: F, t6084: F, t19049: F, t4729: F, t23649: F, t3022: F, t19023: F, t4719: F, t23457: F, t23478: F, t689: F, t76397: F, t905: F, t128: F, t904: F, t23489: F, t23482: F, t23486: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t77386 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2879::<F>(t1583, t1940, t39783, t39786, t39791, t39795, t39799, t49958, t49964, t49982, t63160, t76974, t76976);
        let (t77387, t77400) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2880::<F>(t39807, t39813, t39818, t39823, t40084, t40088, t76977, t76978, t76980, t76986, t76987, t1544, t23111, t23148, t2403, t2404, t40131, t40137, t50080, t61139, t76999, t77000, t77002, t77003, t77004, t77005);
        let t77412 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2881::<F>(t5966, t890, t18435, t18498, t39989, t40150, t4541, t4546, t4556, t50098, t77007, t77008, t77009, t77010, t77011);
        let t77429 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2882::<F>(t5962, t890, t1544, t18850, t18860, t18865, t2403, t27375, t4343, t4433, t4541, t4556, t50866, t63146, t77012, t77013, t77014, t77015, t77020);
        let (t77441, t77455) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2883::<F>(t1544, t4537, t14353, t18268, t2403, t40167, t40171, t40184, t4433, t4541, t50884, t5962, t77024, t77025, t77026, t77027, t77028);
        let t77467 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2884::<F>(t23421, t892, t18865, t18871, t18875, t2403, t77029, t77032, t77036, t77038, t77039, t77040, t77041, t77045, t775);
        let t77472 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2885::<F>(t40076, t40079, t40194, t40198, t77048, t77051, t77053, t77056, t77058, t77059, t77060, t14353, t18392, t18850, t18865, t18871, t1940, t198, t207, t23114, t23279, t2403, t2404, t27384, t39419, t39422, t39483, t39741, t39744, t39747, t39750, t40067, t40072, t40099, t40103, t40115, t4343, t4537, t4541, t4546, t4556, t50048, t50874, t5966, t61033, t75970, t75990, t76012, t76038, t76055, t76077, t76421, t765, t76890, t76893, t76932, t76935, t76936, t76942, t76944, t76946, t76948, t76950, t76951, t76952, t76954, t76963, t76966, t76988, t76991, t76992, t76995, t76997, t76998, t77021, t77023, t77298, t77326, t77333, t77347, t77360, t77373, t77381, t77386, t77387, t77400, t77412, t77429, t77441, t77455, t77467, t775, t890, t892);
        let (t77481, t77492, t77494, t77496, t77498, t77499) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2886::<F>(t2, t580, t6084, t19049, t4729, t23649, t3022, t19023, t4719, t23457, t23478, t689);
        let (t77501, t77503) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2887::<F>(t76397, t905, t128, t904);
        let t77505 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2888::<F>(t23489, t689);
        let t77507 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2889::<F>(t23482, t689);
        let t77509 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2890::<F>(t23486, t689);
    (t77472, t77481, t77492, t77494, t77496, t77498, t77499, t77501, t77503, t77505, t77507, t77509)
}
