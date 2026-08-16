//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta514 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1974;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1975;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1976;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1977;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1978;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta514<F: Float>(t22055: F, t3440: F, t20234: F, t3441: F, t1177: F, t21745: F, t4900: F, t15390: F, t18469: F, t18416: F, t4904: F, t18409: F, t4919: F, t18427: F, t11547: F, t11546: F, t1174: F, t15265: F, t1710: F, t1717: F, t18321: F, t22035: F, t22041: F, t22047: F, t22052: F, t3447: F, t4889: F, t6120: F, t6141: F, t6147: F, t21749: F, t4908: F, t18420: F, t20246: F, t338: F, t11556: F, t15300: F, t15364: F, t15376: F, t18447: F, t18452: F, t18455: F, t18458: F, t18460: F, t18489: F, t18530: F, t18533: F, t18536: F, t463: F, t6123: F, t6127: F, t6131: F, t225: F, t68: F, t484: F, t1196: F, t20217: F, t974: F, t11848: F, t11759: F, t11649: F, t1726: F, t18310: F, t18312: F, t18314: F, t18325: F, t18327: F, t18330: F, t18333: F, t22012: F, t22015: F, t488: F, t6178: F, t6184: F, t6188: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22056, t22059, t22060, t22063, t22066, t22069, t22072) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1974::<F>(t22055, t3440, t20234, t3441, t1177, t21745, t4900, t15390, t18469, t18416, t4904, t18409, t4919);
        let (t22081, t22085) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1975::<F>(t18427, t4919, t11547, t20234, t11546, t1174, t15265, t1710, t1717, t18321, t22035, t22041, t22047, t22052, t22056, t22060, t22063, t22066, t22069, t22072, t3447, t4889, t6120, t6141, t6147);
        let (t22104, t22112) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1976::<F>(t21749, t4908, t18420, t4904, t20246, t338, t11556, t15300, t15364, t15376, t18447, t18452, t18455, t18458, t18460, t18489, t18530, t18533, t18536, t3447, t463, t4889, t6123, t6127, t6131);
        let (t22113, t22114, t22115, t22116, t22119, t22128, t22129, t22132) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1977::<F>(t22085, t22112, t225, t68, t484, t1177, t21749, t1196, t20217, t974, t11848, t20234);
        let (t22136, t22152) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1978::<F>(t22132, t974, t11759, t20234, t21745, t3440, t11649, t1174, t1726, t18310, t18312, t18314, t18321, t18325, t18327, t18330, t18333, t22012, t22015, t22116, t22119, t22129, t488, t4889, t6178, t6184, t6188);
    (t22059, t22081, t22104, t22113, t22114, t22115, t22116, t22128, t22132, t22136, t22152)
}
