//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1527;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1528;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1529;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1530;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1531;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1532;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1533;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1534;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1535;
use chunk9::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1536;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta402<F: Float>(t2970: F, t5828: F, t973: F, t16558: F, t978: F, t977: F, t343: F, t5836: F, t984: F, t4546: F, t10231: F, t5817: F, t13861: F, t4531: F, t17178: F, t4510: F, t2989: F, t5398: F, t2988: F, t10186: F, t13830: F, t13850: F, t2960: F, t2986: F, t5818: F, t5821: F, t5829: F, t2987: F, t2990: F, t5842: F, t13847: F, t4514: F, t17167: F, t4518: F, t17171: F, t10254: F, t5392: F, t17183: F, t135: F, t5844: F, t10295: F, t10296: F, t13642: F, t13921: F, t13922: F, t13923: F, t17241: F, t17244: F, t17247: F, t17250: F, t17253: F, t17256: F, t17280: F, t17286: F, t17288: F, t17290: F, t17293: F, t340: F, t974: F, t5838: F, t5839: F, t5845: F, t17157: F, t17161: F, t13798: F, t17152: F, t10236: F, t10235: F, t13851: F, t10287: F, t10333: F, t10339: F, t13893: F, t13896: F, t13907: F, t13909: F, t13915: F, t17766: F, t225: F, t68: F, t369: F, t10457: F, t248: F, t5677: F, t1041: F, t1044: F, t17187: F, t14084: F, t14085: F, t14117: F, t14508: F, t14511: F, t1622: F, t17734: F, t17738: F, t3048: F, t3117: F, t3130: F, t378: F, t4596: F, t4600: F, t4636: F, t4644: F, t5857: F, t5861: F, t3051: F, t5681: F, t1616: F, t4338: F, t10408: F, t1409: F, t14219: F, t14218: F, t3071: F, t2940: F, t5804: F, t14459: F, t4496: F, t959: F, t17194: F, t17197: F, t17209: F, t17301: F, t17303: F, t17306: F, t17372: F, t17374: F, t17377: F, t17379: F, t17425: F, t17427: F, t17561: F, t17563: F, t17568: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17770, t17773, t17778, t17783) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1527::<F>(t2970, t5828, t973, t16558, t978, t977, t343, t5836, t984, t4546, t10231, t5817);
        let t17798 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1528::<F>(t17783, t973, t13861, t4531, t17178, t4510, t2989, t5398, t2988, t10186, t13830, t13850, t17770, t17773, t17778, t2960, t2986, t5818, t5821, t5829);
        let (t17801, t17805, t17809, t17811, t17814, t17817) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1529::<F>(t2987, t5836, t2990, t5842, t13847, t4514, t2986, t17167, t4518, t17171, t10254, t5392);
        let (t17818, t17821, t17827, t17841) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1530::<F>(t17817, t2988, t17183, t4518, t135, t5844, t973, t10295, t10296, t13642, t13921, t13922, t13923, t17241, t17244, t17247, t17250, t17253, t17256, t17280, t17286, t17288, t17290, t17293);
        let t17852 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1531::<F>(t17841, t340, t343, t974, t135, t5838, t973, t17801, t17805, t17809, t17811, t17814, t17818, t17821, t17827, t2960, t2986, t5839, t5845);
        let t17873 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1532::<F>(t17157, t4510, t17161, t13798, t17152, t10236, t5392, t10235, t13851, t4514, t10287, t10333, t10339, t13893, t13896, t13907, t13909, t13915, t2986);
        let (t17875, t17876, t17878, t17885, t17890) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1533::<F>(t17766, t17798, t17852, t17873, t225, t68, t369, t10457, t248, t5677, t1041, t1044, t17187);
        let t17900 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1534::<F>(t1041, t14084, t14085, t14117, t14508, t14511, t1622, t17734, t17738, t17878, t17885, t17890, t3048, t3117, t3130, t378, t4596, t4600, t4636, t4644, t5857, t5861, t973);
        let (t17907, t17920, t17925, t17929) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1535::<F>(t248, t3051, t5681, t1041, t1616, t4338, t10408, t1409, t14219, t14218, t3071, t2940, t5804);
        let (t17932, t17933) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1536::<F>(t14459, t4496, t959, t17194, t17197, t17209, t17301, t17303, t17306, t17372, t17374, t17377, t17379, t17425, t17427, t17561, t17563, t17568, t17929);
    (t17875, t17876, t17900, t17907, t17920, t17925, t17929, t17932, t17933)
}
