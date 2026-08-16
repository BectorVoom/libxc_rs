//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta949 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3137;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3138;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3139;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3140;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3141;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta949(t17529: f64, t20786: f64, t1042: f64, t1247: f64, t1250: f64, t12956: f64, t1715: f64, t17505: f64, t20809: f64, t20876: f64, t20880: f64, t21242: f64, t24759: f64, t24773: f64, t3708: f64, t3711: f64, t482: f64, t5056: f64, t5268: f64, t5304: f64, t58927: f64, t6619: f64, t69742: f64, t82368: f64, t82422: f64, t21102: f64, t5265: f64, t20816: f64, t5274: f64, t1261: f64, t17569: f64, t17609: f64, t20825: f64, t20907: f64, t20914: f64, t21143: f64, t24808: f64, t3647: f64, t5270: f64, t5279: f64, t5381: f64, t6625: f64, t69906: f64, t80045: f64, t80050: f64, t13042: f64, t24663: f64, t3172: f64, t5284: f64, t6587: f64, t1774: f64, t20900: f64, t606: f64, t12910: f64, t12916: f64, t24740: f64, t1248: f64, t24633: f64, t12787: f64, t12856: f64, t17426: f64, t17729: f64, t20292: f64, t24731: f64, t3718: f64, t3720: f64, t4181: f64, t44225: f64, t44578: f64, t5330: f64, t5335: f64, t5343: f64, t57005: f64, t68289: f64, t72326: f64, t72370: f64, t82293: f64, t1214: f64, t24616: f64, t5245: f64, t6573: f64, t5378: f64, t21192: f64, t5391: f64, t17344: f64, t17396: f64, t17412: f64, t1808: f64, t20986: f64, t21037: f64, t21042: f64, t247: f64, t3719: f64, t5384: f64, t5397: f64, t57520: f64, t6673: f64, t70647: f64, t71590: f64, t82207: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t82438 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3137(t17529, t20786, t1042, t1247, t1250, t12956, t1715, t17505, t20809, t20876, t20880, t21242, t24759, t24773, t3708, t3711, t482, t5056, t5268, t5304, t58927, t6619, t69742, t82368, t82422);
        let t82467 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3138(t21102, t5265, t20816, t5274, t1042, t1261, t17569, t17609, t20825, t20907, t20914, t21143, t24808, t3647, t5268, t5270, t5279, t5381, t6625, t69906, t80045, t80050);
        let (t82469, t82471, t82476, t82481) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3139(t13042, t24663, t3172, t5284, t6587, t1774, t20900, t606);
        let (t82493, t82510) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3140(t12910, t12916, t24740, t1248, t24633, t1250, t12787, t12856, t17426, t17729, t20292, t24731, t3718, t3720, t4181, t44225, t44578, t5330, t5335, t5343, t57005, t68289, t72326, t72370, t82293, t82469, t82471, t82476, t82481);
        let (t82514, t82525, t82542) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3141(t1214, t24616, t5245, t6573, t21143, t5378, t21192, t5391, t17344, t17396, t17412, t1808, t20986, t21037, t21042, t247, t3719, t5381, t5384, t5397, t57520, t6673, t70647, t71590, t82207);
    (t82438, t82467, t82471, t82476, t82481, t82493, t82510, t82514, t82525, t82542)
}
