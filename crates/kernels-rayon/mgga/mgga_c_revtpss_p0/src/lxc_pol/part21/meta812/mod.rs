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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2969;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2970;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2971;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2972;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2973;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2974;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2975;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2976;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta812(t15648: f64, t999: f64, t1011: f64, t1655: f64, t2438: f64, t1014: f64, t4579: f64, t697: f64, t3252: f64, t4574: f64, t16020: f64, t1062: f64, t15887: f64, t11921: f64, t15837: f64, t247: f64, t4837: f64, t11267: f64, t4878: f64, t11263: f64, t4879: f64, t1047: f64, t1068: f64, t11714: f64, t11875: f64, t15606: f64, t3116: f64, t3117: f64, t42830: f64, t4831: f64, t4893: f64, t11773: f64, t3278: f64, t11687: f64, t11774: f64, t12021: f64, t15584: f64, t15689: f64, t15691: f64, t15700: f64, t15701: f64, t15703: f64, t15809: f64, t16009: f64, t16013: f64, t1671: f64, t3095: f64, t3241: f64, t42235: f64, t42425: f64, t42699: f64, t42710: f64, t4786: f64, t4869: f64, t4875: f64, t53846: f64, t11922: f64, t15898: f64, t16003: f64, t16006: f64, t42712: f64, t42716: f64, t42719: f64, t42724: f64, t42727: f64, t42740: f64, t42745: f64, t4919: f64, t51873: f64, t15728: f64, t15827: f64, t11672: f64, t15984: f64, t1042: f64, t11231: f64, t11637: f64, t11703: f64, t11994: f64, t16089: f64, t16095: f64, t16138: f64, t16167: f64, t20094: f64, t20099: f64, t2862: f64, t3092: f64, t3127: f64, t42417: f64, t42695: f64, t42754: f64, t42756: f64, t4783: f64, t4912: f64, t51840: f64, t51844: f64, t51846: f64, t52141: f64, t52146: f64, t52150: f64, t52153: f64, t52156: f64, t52159: f64, t52162: f64, t52166: f64, t52170: f64, t52174: f64, t52176: f64, t52178: f64, t52180: f64, t52182: f64, t52185: f64, t52187: f64, t52194: f64, t52196: f64, t52201: f64, t52204: f64, t52207: f64, t52209: f64, t52211: f64, t52213: f64, t52216: f64, t52218: f64, t52221: f64, t52223: f64, t52226: f64, t52229: f64, t52231: f64, t52235: f64, t52237: f64, t52242: f64, t52245: f64, t52860: f64, t52863: f64, t52481: f64, t52486: f64, t52488: f64, t52490: f64, t52492: f64, t52495: f64, t52499: f64, t52502: f64, t52507: f64, t52865: f64, t52867: f64, t52869: f64, t52874: f64, t52876: f64, t52880: f64, t52882: f64, t52885: f64, t52887: f64, t52889: f64, t52897: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54112, t54118, t54123, t54127, t54130, t54137) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2969(t15648, t999, t1011, t1655, t2438, t1014, t4579, t697, t3252, t4574, t16020, t1062, t15887);
        let t54149 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2970(t11921, t15837, t247, t4837, t11267, t4878, t11263, t4879, t1047, t1068, t11714, t11875, t15606, t3116, t3117, t42830, t4831, t4893, t54112, t54118, t54123, t54127, t54130, t54137);
        let (t54166, t54176) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2971(t11773, t3278, t11687, t11774, t12021, t15584, t15689, t15691, t15700, t15701, t15703, t15809, t16009, t16013, t1671, t3095, t3241, t42235, t42425, t42699, t42710, t4786, t4869, t4875, t53846);
        let t54195 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2972(t11875, t11922, t15898, t1011, t16003, t16006, t3241, t42712, t42716, t42719, t42724, t42727, t42740, t42745, t4919, t51873);
        let t54224 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2973(t15728, t15827, t11672, t15984, t1042, t11231, t11637, t11703, t11994, t16089, t16095, t16138, t16167, t20094, t20099, t2862, t3092, t3127, t42417, t42695, t42754, t42756, t4783, t4912);
        let t54230 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2974(t51840, t51844, t51846, t52141, t52146, t52150, t52153, t52156, t52159, t52162, t52166, t52170, t52174, t52176, t52178, t52180, t52182, t52185, t52187, t52194);
        let t54231 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2975(t52196, t52201, t52204, t52207, t52209, t52211, t52213, t52216, t52218, t52221, t52223, t52226, t52229, t52231, t52235, t52237, t52242, t52245, t52860, t52863);
        let t54233 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2976(t52481, t52486, t52488, t52490, t52492, t52495, t52499, t52502, t52507, t52865, t52867, t52869, t52874, t52876, t52880, t52882, t52885, t52887, t52889, t52897);
    (t54112, t54130, t54149, t54166, t54176, t54195, t54224, t54230, t54231, t54233)
}
