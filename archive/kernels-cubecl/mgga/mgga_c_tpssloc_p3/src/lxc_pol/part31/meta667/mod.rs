//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta667 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1960;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1961;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1962;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1963;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1964;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta667<F: Float>(t12725: F, t1458: F, t1774: F, t1849: F, t1983: F, t19924: F, t20085: F, t2096: F, t22574: F, t2314: F, t24432: F, t24995: F, t26114: F, t26179: F, t26558: F, t26870: F, t26967: F, t27163: F, t27188: F, t27215: F, t28030: F, t29201: F, t29205: F, t29243: F, t33234: F, t4034: F, t4073: F, t652: F, t6876: F, t7057: F, t7217: F, t74060: F, t7458: F, t7796: F, t7802: F, t9016: F, t97804: F, t97911: F, t12461: F, t7939: F, t29376: F, t532: F, t193: F, t200: F, t7844: F, t1877: F, t2057: F, t24191: F, t25015: F, t25021: F, t2522: F, t25366: F, t25392: F, t26563: F, t26744: F, t28252: F, t7110: F, t7114: F, t92319: F, t97956: F, t97990: F, t98004: F, t98008: F, t98059: F, t98079: F, t98094: F, t99049: F, t99056: F, t26756: F, t98069: F, t2219: F, t7845: F, t2752: F, t29105: F, t99053: F, t1408: F, t24339: F, t25028: F, t25381: F, t26740: F, t28456: F, t28462: F, t29106: F, t6542: F, t6671: F, t84800: F, t98012: F, t98020: F, t98086: F, t98112: F, t99060: F, t24344: F, t28241: F, t28249: F, t28972: F, t4314: F, t46341: F, t5397: F, t7475: F, t7545: F, t84797: F, t92276: F, t98000: F, t98031: F, t98046: F, t98050: F, t98065: F, t98082: F, t98091: F, t98103: F, t13042: F, t17064: F, t2054: F, t259: F, t26713: F, t4142: F, t4273: F, t59503: F, t7087: F, t7823: F, t7830: F, t86870: F, t92375: F, t92382: F, t92390: F, t92393: F, t98117: F, t98122: F, t98125: F, t98135: F, t98148: F, t98153: F, t98158: F, t98164: F, t98172: F, t98181: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t101134 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1960::<F>(t12725, t1458, t1774, t1849, t1983, t19924, t20085, t2096, t22574, t2314, t24432, t24995, t26114, t26179, t26558, t26870, t26967, t27163, t27188, t27215, t28030, t29201, t29205, t29243, t33234, t4034, t4073, t652, t6876, t7057, t7217, t74060, t7458, t7796, t7802, t9016, t97804, t97911);
        let (t101138, t101150, t101196, t101209) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1961::<F>(t12461, t7939, t29376, t532, t193, t200, t7844, t1877, t2057, t24191, t25015, t25021, t2522, t25366, t25392, t26563, t26744, t28252, t7110, t7114, t92319, t97956, t97990, t98004, t98008, t98059, t98079, t98094, t99049, t99056);
        let (t101211, t101220, t101226, t101241, t101248) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1962::<F>(t26756, t98069, t1877, t2219, t7845, t2752, t29105, t24191, t99053, t1408, t2057, t24339, t25028, t2522, t25381, t26563, t26740, t26744, t28456, t28462, t29106, t6542, t6671, t7114, t84800, t98012, t98020, t98086, t98112, t99060);
        let t101283 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1963::<F>(t1877, t2057, t24344, t2522, t26740, t26756, t28241, t28249, t28972, t4314, t46341, t5397, t7110, t7114, t7475, t7545, t84797, t92276, t98000, t98031, t98046, t98050, t98065, t98082, t98091, t98103);
        let t101335 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1964::<F>(t13042, t17064, t2054, t259, t26713, t4142, t4273, t59503, t7087, t7823, t7830, t86870, t92375, t92382, t92390, t92393, t98117, t98122, t98125, t98135, t98148, t98153, t98158, t98164, t98172, t98181);
    (t101134, t101138, t101150, t101196, t101209, t101211, t101220, t101226, t101241, t101248, t101283, t101335)
}
