//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1531;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1532;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1533;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1534;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1535;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta402(t17817: f64, t2988: f64, t17183: f64, t4518: f64, t135: f64, t5844: f64, t973: f64, t10295: f64, t10296: f64, t13642: f64, t13921: f64, t13922: f64, t13923: f64, t17241: f64, t17244: f64, t17247: f64, t17250: f64, t17253: f64, t17256: f64, t17280: f64, t17286: f64, t17288: f64, t17290: f64, t17293: f64, t340: f64, t343: f64, t974: f64, t5838: f64, t17801: f64, t17805: f64, t17809: f64, t17811: f64, t17814: f64, t2960: f64, t2986: f64, t5839: f64, t5845: f64, t17157: f64, t4510: f64, t17161: f64, t13798: f64, t17152: f64, t10236: f64, t5392: f64, t10235: f64, t13851: f64, t4514: f64, t10287: f64, t10333: f64, t10339: f64, t13893: f64, t13896: f64, t13907: f64, t13909: f64, t13915: f64, t17766: f64, t17798: f64, t225: f64, t68: f64, t369: f64, t10457: f64, t248: f64, t5677: f64, t1041: f64, t1044: f64, t17187: f64, t14084: f64, t14085: f64, t14117: f64, t14508: f64, t14511: f64, t1622: f64, t17734: f64, t17738: f64, t3048: f64, t3117: f64, t3130: f64, t378: f64, t4596: f64, t4600: f64, t4636: f64, t4644: f64, t5857: f64, t5861: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17818, t17821, t17827, t17841) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1531(t17817, t2988, t17183, t4518, t135, t5844, t973, t10295, t10296, t13642, t13921, t13922, t13923, t17241, t17244, t17247, t17250, t17253, t17256, t17280, t17286, t17288, t17290, t17293);
        let (t17843, t17852) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1532(t17841, t340, t343, t974, t135, t5838, t973, t17801, t17805, t17809, t17811, t17814, t17818, t17821, t17827, t2960, t2986, t5839, t5845);
        let t17873 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1533(t17157, t4510, t17161, t13798, t17152, t10236, t5392, t10235, t13851, t4514, t10287, t10333, t10339, t13893, t13896, t13907, t13909, t13915, t2986);
        let (t17875, t17876, t17877, t17878, t17884, t17885, t17890) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1534(t17766, t17798, t17852, t17873, t225, t68, t369, t10457, t248, t5677, t1041, t1044, t17187);
        let t17900 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1535(t1041, t14084, t14085, t14117, t14508, t14511, t1622, t17734, t17738, t17878, t17885, t17890, t3048, t3117, t3130, t378, t4596, t4600, t4636, t4644, t5857, t5861, t973);
    (t17841, t17843, t17875, t17876, t17877, t17884, t17890, t17900)
}
