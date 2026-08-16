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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta402(t2970: f64, t5828: f64, t973: f64, t16558: f64, t978: f64, t977: f64, t343: f64, t5836: f64, t984: f64, t4546: f64, t10231: f64, t5817: f64, t13861: f64, t4531: f64, t17178: f64, t4510: f64, t2989: f64, t5398: f64, t2988: f64, t10186: f64, t13830: f64, t13850: f64, t2960: f64, t2986: f64, t5818: f64, t5821: f64, t5829: f64, t2987: f64, t2990: f64, t5842: f64, t13847: f64, t4514: f64, t17167: f64, t4518: f64, t17171: f64, t10254: f64, t5392: f64, t17183: f64, t135: f64, t5844: f64, t10295: f64, t10296: f64, t13642: f64, t13921: f64, t13922: f64, t13923: f64, t17241: f64, t17244: f64, t17247: f64, t17250: f64, t17253: f64, t17256: f64, t17280: f64, t17286: f64, t17288: f64, t17290: f64, t17293: f64, t340: f64, t974: f64, t5838: f64, t5839: f64, t5845: f64, t17157: f64, t17161: f64, t13798: f64, t17152: f64, t10236: f64, t10235: f64, t13851: f64, t10287: f64, t10333: f64, t10339: f64, t13893: f64, t13896: f64, t13907: f64, t13909: f64, t13915: f64, t17766: f64, t225: f64, t68: f64, t369: f64, t10457: f64, t248: f64, t5677: f64, t1041: f64, t1044: f64, t17187: f64, t14084: f64, t14085: f64, t14117: f64, t14508: f64, t14511: f64, t1622: f64, t17734: f64, t17738: f64, t3048: f64, t3117: f64, t3130: f64, t378: f64, t4596: f64, t4600: f64, t4636: f64, t4644: f64, t5857: f64, t5861: f64, t3051: f64, t5681: f64, t1616: f64, t4338: f64, t10408: f64, t1409: f64, t14219: f64, t14218: f64, t3071: f64, t2940: f64, t5804: f64, t14459: f64, t4496: f64, t959: f64, t17194: f64, t17197: f64, t17209: f64, t17301: f64, t17303: f64, t17306: f64, t17372: f64, t17374: f64, t17377: f64, t17379: f64, t17425: f64, t17427: f64, t17561: f64, t17563: f64, t17568: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17770, t17773, t17778, t17783) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1527(t2970, t5828, t973, t16558, t978, t977, t343, t5836, t984, t4546, t10231, t5817);
        let t17798 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1528(t17783, t973, t13861, t4531, t17178, t4510, t2989, t5398, t2988, t10186, t13830, t13850, t17770, t17773, t17778, t2960, t2986, t5818, t5821, t5829);
        let (t17801, t17805, t17809, t17811, t17814, t17817) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1529(t2987, t5836, t2990, t5842, t13847, t4514, t2986, t17167, t4518, t17171, t10254, t5392);
        let (t17818, t17821, t17827, t17841) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1530(t17817, t2988, t17183, t4518, t135, t5844, t973, t10295, t10296, t13642, t13921, t13922, t13923, t17241, t17244, t17247, t17250, t17253, t17256, t17280, t17286, t17288, t17290, t17293);
        let t17852 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1531(t17841, t340, t343, t974, t135, t5838, t973, t17801, t17805, t17809, t17811, t17814, t17818, t17821, t17827, t2960, t2986, t5839, t5845);
        let t17873 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1532(t17157, t4510, t17161, t13798, t17152, t10236, t5392, t10235, t13851, t4514, t10287, t10333, t10339, t13893, t13896, t13907, t13909, t13915, t2986);
        let (t17875, t17876, t17878, t17885, t17890) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1533(t17766, t17798, t17852, t17873, t225, t68, t369, t10457, t248, t5677, t1041, t1044, t17187);
        let t17900 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1534(t1041, t14084, t14085, t14117, t14508, t14511, t1622, t17734, t17738, t17878, t17885, t17890, t3048, t3117, t3130, t378, t4596, t4600, t4636, t4644, t5857, t5861, t973);
        let (t17907, t17920, t17925, t17929) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1535(t248, t3051, t5681, t1041, t1616, t4338, t10408, t1409, t14219, t14218, t3071, t2940, t5804);
        let (t17932, t17933) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1536(t14459, t4496, t959, t17194, t17197, t17209, t17301, t17303, t17306, t17372, t17374, t17377, t17379, t17425, t17427, t17561, t17563, t17568, t17929);
    (t17875, t17876, t17900, t17907, t17920, t17925, t17929, t17932, t17933)
}
