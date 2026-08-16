//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta713 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2312;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2313;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2314;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2315;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2316;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta713(t40804: f64, t40806: f64, t40790: f64, t40793: f64, t40797: f64, t40799: f64, t40801: f64, t40803: f64, t46311: f64, t67214: f64, t67215: f64, t12939: f64, t16716: f64, t3966: f64, t40: f64, t12908: f64, t20749: f64, t12923: f64, t4194: f64, t5398: f64, t20800: f64, t262: f64, t10143: f64, t20778: f64, t13115: f64, t16586: f64, t12950: f64, t1430: f64, t16558: f64, t16637: f64, t17635: f64, t20217: f64, t20234: f64, t2291: f64, t4104: f64, t607: f64, t67060: f64, t75: f64, t767: f64, zeta_threshold: f64, t52: f64, t12961: f64, t1431: f64, t16649: f64, t2298: f64, t4111: f64, t771: f64, t78: f64, t12895: f64, t193: f64, t20756: f64, t2522: f64, t39549: f64, t39563: f64, t4314: f64, t5527: f64, t766: f64, t776: f64, t868: f64, t870: f64, t21038: f64, t225: f64, t10110: f64, t1527: f64, t1528: f64, t17049: f64, t17057: f64, t17064: f64, t17092: f64, t21013: f64, t21049: f64, t21054: f64, t259: f64, t2597: f64, t2713: f64, t2718: f64, t40890: f64, t4147: f64, t4273: f64, t4300: f64, t5636: f64, t5657: f64, t59466: f64, t59537: f64, t798: f64, t855: f64, t865: f64, t866: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t67216, t67217, t67218, t67226) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2312(t40804, t40806, t40790, t40793, t40797, t40799, t40801, t40803, t46311, t67214, t67215, t12939, t16716, t3966);
        let (t67228, t67231, t67235, t67239, t67244, t67262) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2313(t40, t12908, t20749, t12923, t4194, t5398, t20800, t262, t10143, t20778, t13115, t16586, t12950, t1430, t16558, t16637, t17635, t20217, t20234, t2291, t3966, t4104, t607, t67060, t75, t767, zeta_threshold);
        let t67280 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2314(t52, t12961, t1431, t16558, t16649, t17635, t20217, t20234, t2298, t3966, t4111, t5398, t607, t67060, t771, t78, zeta_threshold);
        let (t67282, t67286) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2315(t67262, t67280, t12895, t193, t20756, t2522, t39549, t39563, t4314, t5527, t67226, t67228, t67231, t67235, t67239, t67244, t766, t776, t868, t870);
        let t67322 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2316(t21038, t225, t10110, t1527, t1528, t17049, t17057, t17064, t17092, t21013, t21049, t21054, t259, t2597, t2713, t2718, t40890, t4147, t4273, t4300, t5636, t5657, t59466, t59537, t798, t855, t865, t866);
    (t67216, t67217, t67218, t67226, t67228, t67231, t67244, t67282, t67286, t67322)
}
