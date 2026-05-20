//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta949 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3137;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3138;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3139;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3140;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3141;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta949<F: Float>(t17529: F, t20786: F, t1042: F, t1247: F, t1250: F, t12956: F, t1715: F, t17505: F, t20809: F, t20876: F, t20880: F, t21242: F, t24759: F, t24773: F, t3708: F, t3711: F, t482: F, t5056: F, t5268: F, t5304: F, t58927: F, t6619: F, t69742: F, t82368: F, t82422: F, t21102: F, t5265: F, t20816: F, t5274: F, t1261: F, t17569: F, t17609: F, t20825: F, t20907: F, t20914: F, t21143: F, t24808: F, t3647: F, t5270: F, t5279: F, t5381: F, t6625: F, t69906: F, t80045: F, t80050: F, t13042: F, t24663: F, t3172: F, t5284: F, t6587: F, t1774: F, t20900: F, t606: F, t12910: F, t12916: F, t24740: F, t1248: F, t24633: F, t12787: F, t12856: F, t17426: F, t17729: F, t20292: F, t24731: F, t3718: F, t3720: F, t4181: F, t44225: F, t44578: F, t5330: F, t5335: F, t5343: F, t57005: F, t68289: F, t72326: F, t72370: F, t82293: F, t1214: F, t24616: F, t5245: F, t6573: F, t5378: F, t21192: F, t5391: F, t17344: F, t17396: F, t17412: F, t1808: F, t20986: F, t21037: F, t21042: F, t247: F, t3719: F, t5384: F, t5397: F, t57520: F, t6673: F, t70647: F, t71590: F, t82207: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t82438 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3137::<F>(t17529, t20786, t1042, t1247, t1250, t12956, t1715, t17505, t20809, t20876, t20880, t21242, t24759, t24773, t3708, t3711, t482, t5056, t5268, t5304, t58927, t6619, t69742, t82368, t82422);
        let t82467 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3138::<F>(t21102, t5265, t20816, t5274, t1042, t1261, t17569, t17609, t20825, t20907, t20914, t21143, t24808, t3647, t5268, t5270, t5279, t5381, t6625, t69906, t80045, t80050);
        let (t82469, t82471, t82476, t82481) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3139::<F>(t13042, t24663, t3172, t5284, t6587, t1774, t20900, t606);
        let (t82493, t82510) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3140::<F>(t12910, t12916, t24740, t1248, t24633, t1250, t12787, t12856, t17426, t17729, t20292, t24731, t3718, t3720, t4181, t44225, t44578, t5330, t5335, t5343, t57005, t68289, t72326, t72370, t82293, t82469, t82471, t82476, t82481);
        let (t82514, t82525, t82542) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3141::<F>(t1214, t24616, t5245, t6573, t21143, t5378, t21192, t5391, t17344, t17396, t17412, t1808, t20986, t21037, t21042, t247, t3719, t5381, t5384, t5397, t57520, t6673, t70647, t71590, t82207);
    (t82438, t82467, t82471, t82476, t82481, t82493, t82510, t82514, t82525, t82542)
}
