//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1077;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1078;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta318(t21749: f64, t4908: f64, t18420: f64, t4904: f64, t20246: f64, t338: f64, t11556: f64, t15300: f64, t15364: f64, t15376: f64, t18447: f64, t18452: f64, t18455: f64, t18458: f64, t18460: f64, t18489: f64, t18530: f64, t18533: f64, t18536: f64, t3447: f64, t463: f64, t4889: f64, t6123: f64, t6127: f64, t6131: f64, t22085: f64, t225: f64, t68: f64, t484: f64, t1177: f64, t1196: f64, t20217: f64, t974: f64, t11848: f64, t20234: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22090, t22095, t22104, t22112) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1077(t21749, t4908, t18420, t4904, t20246, t338, t11556, t15300, t15364, t15376, t18447, t18452, t18455, t18458, t18460, t18489, t18530, t18533, t18536, t3447, t463, t4889, t6123, t6127, t6131);
        let (t22113, t22114, t22115, t22116, t22119, t22128, t22129, t22132) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1078(t22085, t22112, t225, t68, t484, t1177, t21749, t1196, t20217, t974, t11848, t20234);
    (t22090, t22095, t22104, t22113, t22114, t22115, t22116, t22119, t22128, t22129, t22132)
}
