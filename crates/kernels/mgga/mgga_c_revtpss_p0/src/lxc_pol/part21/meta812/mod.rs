//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta812 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2969;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2970;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2971;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2972;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2973;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2974;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2975;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2976;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta812<F: Float>(t15648: F, t999: F, t1011: F, t1655: F, t2438: F, t1014: F, t4579: F, t697: F, t3252: F, t4574: F, t16020: F, t1062: F, t15887: F, t11921: F, t15837: F, t247: F, t4837: F, t11267: F, t4878: F, t11263: F, t4879: F, t1047: F, t1068: F, t11714: F, t11875: F, t15606: F, t3116: F, t3117: F, t42830: F, t4831: F, t4893: F, t11773: F, t3278: F, t11687: F, t11774: F, t12021: F, t15584: F, t15689: F, t15691: F, t15700: F, t15701: F, t15703: F, t15809: F, t16009: F, t16013: F, t1671: F, t3095: F, t3241: F, t42235: F, t42425: F, t42699: F, t42710: F, t4786: F, t4869: F, t4875: F, t53846: F, t11922: F, t15898: F, t16003: F, t16006: F, t42712: F, t42716: F, t42719: F, t42724: F, t42727: F, t42740: F, t42745: F, t4919: F, t51873: F, t15728: F, t15827: F, t11672: F, t15984: F, t1042: F, t11231: F, t11637: F, t11703: F, t11994: F, t16089: F, t16095: F, t16138: F, t16167: F, t20094: F, t20099: F, t2862: F, t3092: F, t3127: F, t42417: F, t42695: F, t42754: F, t42756: F, t4783: F, t4912: F, t51840: F, t51844: F, t51846: F, t52141: F, t52146: F, t52150: F, t52153: F, t52156: F, t52159: F, t52162: F, t52166: F, t52170: F, t52174: F, t52176: F, t52178: F, t52180: F, t52182: F, t52185: F, t52187: F, t52194: F, t52196: F, t52201: F, t52204: F, t52207: F, t52209: F, t52211: F, t52213: F, t52216: F, t52218: F, t52221: F, t52223: F, t52226: F, t52229: F, t52231: F, t52235: F, t52237: F, t52242: F, t52245: F, t52860: F, t52863: F, t52481: F, t52486: F, t52488: F, t52490: F, t52492: F, t52495: F, t52499: F, t52502: F, t52507: F, t52865: F, t52867: F, t52869: F, t52874: F, t52876: F, t52880: F, t52882: F, t52885: F, t52887: F, t52889: F, t52897: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t54112, t54118, t54123, t54127, t54130, t54137) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2969::<F>(t15648, t999, t1011, t1655, t2438, t1014, t4579, t697, t3252, t4574, t16020, t1062, t15887);
        let t54149 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2970::<F>(t11921, t15837, t247, t4837, t11267, t4878, t11263, t4879, t1047, t1068, t11714, t11875, t15606, t3116, t3117, t42830, t4831, t4893, t54112, t54118, t54123, t54127, t54130, t54137);
        let (t54166, t54176) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2971::<F>(t11773, t3278, t11687, t11774, t12021, t15584, t15689, t15691, t15700, t15701, t15703, t15809, t16009, t16013, t1671, t3095, t3241, t42235, t42425, t42699, t42710, t4786, t4869, t4875, t53846);
        let t54195 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2972::<F>(t11875, t11922, t15898, t1011, t16003, t16006, t3241, t42712, t42716, t42719, t42724, t42727, t42740, t42745, t4919, t51873);
        let t54224 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2973::<F>(t15728, t15827, t11672, t15984, t1042, t11231, t11637, t11703, t11994, t16089, t16095, t16138, t16167, t20094, t20099, t2862, t3092, t3127, t42417, t42695, t42754, t42756, t4783, t4912);
        let t54230 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2974::<F>(t51840, t51844, t51846, t52141, t52146, t52150, t52153, t52156, t52159, t52162, t52166, t52170, t52174, t52176, t52178, t52180, t52182, t52185, t52187, t52194);
        let t54231 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2975::<F>(t52196, t52201, t52204, t52207, t52209, t52211, t52213, t52216, t52218, t52221, t52223, t52226, t52229, t52231, t52235, t52237, t52242, t52245, t52860, t52863);
        let t54233 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2976::<F>(t52481, t52486, t52488, t52490, t52492, t52495, t52499, t52502, t52507, t52865, t52867, t52869, t52874, t52876, t52880, t52882, t52885, t52887, t52889, t52897);
    (t54112, t54130, t54149, t54166, t54176, t54195, t54224, t54230, t54231, t54233)
}
