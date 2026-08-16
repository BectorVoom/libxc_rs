//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta514 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1974;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1975;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1976;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1977;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1978;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta514(t22055: f64, t3440: f64, t20234: f64, t3441: f64, t1177: f64, t21745: f64, t4900: f64, t15390: f64, t18469: f64, t18416: f64, t4904: f64, t18409: f64, t4919: f64, t18427: f64, t11547: f64, t11546: f64, t1174: f64, t15265: f64, t1710: f64, t1717: f64, t18321: f64, t22035: f64, t22041: f64, t22047: f64, t22052: f64, t3447: f64, t4889: f64, t6120: f64, t6141: f64, t6147: f64, t21749: f64, t4908: f64, t18420: f64, t20246: f64, t338: f64, t11556: f64, t15300: f64, t15364: f64, t15376: f64, t18447: f64, t18452: f64, t18455: f64, t18458: f64, t18460: f64, t18489: f64, t18530: f64, t18533: f64, t18536: f64, t463: f64, t6123: f64, t6127: f64, t6131: f64, t225: f64, t68: f64, t484: f64, t1196: f64, t20217: f64, t974: f64, t11848: f64, t11759: f64, t11649: f64, t1726: f64, t18310: f64, t18312: f64, t18314: f64, t18325: f64, t18327: f64, t18330: f64, t18333: f64, t22012: f64, t22015: f64, t488: f64, t6178: f64, t6184: f64, t6188: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22056, t22059, t22060, t22063, t22066, t22069, t22072) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1974(t22055, t3440, t20234, t3441, t1177, t21745, t4900, t15390, t18469, t18416, t4904, t18409, t4919);
        let (t22081, t22085) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1975(t18427, t4919, t11547, t20234, t11546, t1174, t15265, t1710, t1717, t18321, t22035, t22041, t22047, t22052, t22056, t22060, t22063, t22066, t22069, t22072, t3447, t4889, t6120, t6141, t6147);
        let (t22104, t22112) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1976(t21749, t4908, t18420, t4904, t20246, t338, t11556, t15300, t15364, t15376, t18447, t18452, t18455, t18458, t18460, t18489, t18530, t18533, t18536, t3447, t463, t4889, t6123, t6127, t6131);
        let (t22113, t22114, t22115, t22116, t22119, t22128, t22129, t22132) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1977(t22085, t22112, t225, t68, t484, t1177, t21749, t1196, t20217, t974, t11848, t20234);
        let (t22136, t22152) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1978(t22132, t974, t11759, t20234, t21745, t3440, t11649, t1174, t1726, t18310, t18312, t18314, t18321, t18325, t18327, t18330, t18333, t22012, t22015, t22116, t22119, t22129, t488, t4889, t6178, t6184, t6188);
    (t22059, t22081, t22104, t22113, t22114, t22115, t22116, t22128, t22132, t22136, t22152)
}
