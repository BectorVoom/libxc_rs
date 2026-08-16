//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta667 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1960;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1961;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1962;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1963;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1964;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta667(t12725: f64, t1458: f64, t1774: f64, t1849: f64, t1983: f64, t19924: f64, t20085: f64, t2096: f64, t22574: f64, t2314: f64, t24432: f64, t24995: f64, t26114: f64, t26179: f64, t26558: f64, t26870: f64, t26967: f64, t27163: f64, t27188: f64, t27215: f64, t28030: f64, t29201: f64, t29205: f64, t29243: f64, t33234: f64, t4034: f64, t4073: f64, t652: f64, t6876: f64, t7057: f64, t7217: f64, t74060: f64, t7458: f64, t7796: f64, t7802: f64, t9016: f64, t97804: f64, t97911: f64, t12461: f64, t7939: f64, t29376: f64, t532: f64, t193: f64, t200: f64, t7844: f64, t1877: f64, t2057: f64, t24191: f64, t25015: f64, t25021: f64, t2522: f64, t25366: f64, t25392: f64, t26563: f64, t26744: f64, t28252: f64, t7110: f64, t7114: f64, t92319: f64, t97956: f64, t97990: f64, t98004: f64, t98008: f64, t98059: f64, t98079: f64, t98094: f64, t99049: f64, t99056: f64, t26756: f64, t98069: f64, t2219: f64, t7845: f64, t2752: f64, t29105: f64, t99053: f64, t1408: f64, t24339: f64, t25028: f64, t25381: f64, t26740: f64, t28456: f64, t28462: f64, t29106: f64, t6542: f64, t6671: f64, t84800: f64, t98012: f64, t98020: f64, t98086: f64, t98112: f64, t99060: f64, t24344: f64, t28241: f64, t28249: f64, t28972: f64, t4314: f64, t46341: f64, t5397: f64, t7475: f64, t7545: f64, t84797: f64, t92276: f64, t98000: f64, t98031: f64, t98046: f64, t98050: f64, t98065: f64, t98082: f64, t98091: f64, t98103: f64, t13042: f64, t17064: f64, t2054: f64, t259: f64, t26713: f64, t4142: f64, t4273: f64, t59503: f64, t7087: f64, t7823: f64, t7830: f64, t86870: f64, t92375: f64, t92382: f64, t92390: f64, t92393: f64, t98117: f64, t98122: f64, t98125: f64, t98135: f64, t98148: f64, t98153: f64, t98158: f64, t98164: f64, t98172: f64, t98181: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t101134 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1960(t12725, t1458, t1774, t1849, t1983, t19924, t20085, t2096, t22574, t2314, t24432, t24995, t26114, t26179, t26558, t26870, t26967, t27163, t27188, t27215, t28030, t29201, t29205, t29243, t33234, t4034, t4073, t652, t6876, t7057, t7217, t74060, t7458, t7796, t7802, t9016, t97804, t97911);
        let (t101138, t101150, t101196, t101209) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1961(t12461, t7939, t29376, t532, t193, t200, t7844, t1877, t2057, t24191, t25015, t25021, t2522, t25366, t25392, t26563, t26744, t28252, t7110, t7114, t92319, t97956, t97990, t98004, t98008, t98059, t98079, t98094, t99049, t99056);
        let (t101211, t101220, t101226, t101241, t101248) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1962(t26756, t98069, t1877, t2219, t7845, t2752, t29105, t24191, t99053, t1408, t2057, t24339, t25028, t2522, t25381, t26563, t26740, t26744, t28456, t28462, t29106, t6542, t6671, t7114, t84800, t98012, t98020, t98086, t98112, t99060);
        let t101283 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1963(t1877, t2057, t24344, t2522, t26740, t26756, t28241, t28249, t28972, t4314, t46341, t5397, t7110, t7114, t7475, t7545, t84797, t92276, t98000, t98031, t98046, t98050, t98065, t98082, t98091, t98103);
        let t101335 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1964(t13042, t17064, t2054, t259, t26713, t4142, t4273, t59503, t7087, t7823, t7830, t86870, t92375, t92382, t92390, t92393, t98117, t98122, t98125, t98135, t98148, t98153, t98158, t98164, t98172, t98181);
    (t101134, t101138, t101150, t101196, t101209, t101211, t101220, t101226, t101241, t101248, t101283, t101335)
}
