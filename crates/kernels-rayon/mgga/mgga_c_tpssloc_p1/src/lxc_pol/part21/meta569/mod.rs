//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2279;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2280;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2281;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2282;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta569(t225: f64, t6151: f64, t6153: f64, t6239: f64, t1720: f64, t5052: f64, t1751: f64, t4940: f64, t18571: f64, t491: f64, t1252: f64, t14972: f64, t14980: f64, t15797: f64, t1761: f64, t3487: f64, t3593: f64, t4945: f64, t498: f64, t5055: f64, t5089: f64, t6244: f64, t19231: f64, t1256: f64, t18247: f64, t18249: f64, t18251: f64, t18257: f64, t18261: f64, t18264: f64, t18268: f64, t18270: f64, t18273: f64, t18278: f64, t18282: f64, t18285: f64, t18672: f64, t18676: f64, t18679: f64, t18909: f64, t18913: f64, t193: f64, t336: f64, t4700: f64, t5091: f64, t5095: f64, t3640: f64, t6270: f64, t11947: f64, t6274: f64, t1254: f64, t18682: f64, t18685: f64, t18688: f64, t18690: f64, t18692: f64, t18694: f64, t18696: f64, t18837: f64, t18839: f64, t18917: f64, t18920: f64, t18922: f64, t18924: f64, t18928: f64, t18930: f64, t18932: f64, t18936: f64, t18938: f64, t28: f64, t265: f64, t504: f64, t17133: f64, t1081: f64, t1260: f64, t1409: f64, t1649: f64, t16558: f64, t17141: f64, t1768: f64, t18196: f64, t3966: f64, t4324: f64, t506: f64, t5099: f64, t52: f64, t5398: f64, t5669: f64, t5966: f64, t607: f64, t6279: f64, t873: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19232, t19234, t19249, t19253, t19256, t19259, t19261) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2279(t225, t6151, t6153, t6239, t1720, t5052, t1751, t4940, t18571, t491, t1252, t14972, t14980, t15797, t1761, t3487, t3593, t4945, t498, t5055, t5089, t6244);
        let (t19262, t19266) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2280(t19231, t19261, t1256, t18247, t18249, t18251, t18257, t18261, t18264, t18268, t18270, t18273, t18278, t18282, t18285, t18672, t18676, t18679, t18909, t18913, t193, t336, t4700, t5091, t5095);
        let (t19267, t19270, t19274) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2281(t3640, t6270, t11947, t6274, t1254, t18682, t18685, t18688, t18690, t18692, t18694, t18696, t18837, t18839, t18917, t18920, t18922, t18924, t18928, t18930, t18932, t18936, t18938, t4700);
        let (t19276, t19288) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2282(t28, t265, t504, t17133, t19266, t19274, t1081, t1260, t1409, t1649, t16558, t17141, t1768, t18196, t3966, t4324, t506, t5099, t52, t5398, t5669, t5966, t607, t6279, t873, dens_threshold, rho1, zeta_threshold);
    (t19232, t19234, t19249, t19253, t19256, t19259, t19262, t19267, t19270, t19276, t19288)
}
