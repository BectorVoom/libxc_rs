//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta523 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1989;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1990;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1991;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta523(t1755: f64, t22368: f64, t22364: f64, t3625: f64, t22327: f64, t493: f64, t22243: f64, t491: f64, t1246: f64, t1751: f64, t6218: f64, t11881: f64, t11888: f64, t11914: f64, t1244: f64, t15027: f64, t15245: f64, t1729: f64, t1756: f64, t1758: f64, t19201: f64, t22114: f64, t22341: f64, t22349: f64, t22355: f64, t22358: f64, t22361: f64, t22365: f64, t3610: f64, t3624: f64, t470: f64, t494: f64, t5064: f64, t6168: f64, t6253: f64, t6257: f64, t6261: f64, t6263: f64, t6265: f64, t1241: f64, t22113: f64, t1238: f64, t1761: f64, t19232: f64, t19234: f64, t19249: f64, t22004: f64, t22008: f64, t22328: f64, t22334: f64, t22337: f64, t4945: f64, t498: f64, t5055: f64, t6244: f64, t6268: f64, t1256: f64, t1763: f64, t19267: f64, t193: f64, t21956: f64, t21958: f64, t21960: f64, t21963: f64, t21990: f64, t22224: f64, t22226: f64, t22231: f64, t22235: f64, t22239: f64, t22241: f64, t336: f64, t4700: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22369, t22372, t22375, t22386, t22387, t22389, t22390, t22393) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1989(t1755, t22368, t22364, t3625, t22327, t493, t22243, t491, t1246, t1751, t6218, t11881, t11888, t11914, t1244, t15027, t15245, t1729, t1756, t1758, t19201, t22114, t22341, t22349, t22355, t22358, t22361, t22365, t3610, t3624, t470, t494, t5064, t6168, t6253, t6257, t6261, t6263, t6265);
        let (t22394, t22398, t22408) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1990(t1241, t22393, t22113, t491, t1238, t1761, t19232, t19234, t19249, t22004, t22008, t22328, t22334, t22337, t4945, t498, t5055, t6244, t6268);
        let t22412 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1991(t1256, t1763, t19267, t193, t21956, t21958, t21960, t21963, t21990, t22224, t22226, t22231, t22235, t22239, t22241, t22408, t336, t4700);
    (t22369, t22372, t22375, t22386, t22387, t22389, t22390, t22393, t22394, t22398, t22408, t22412)
}
