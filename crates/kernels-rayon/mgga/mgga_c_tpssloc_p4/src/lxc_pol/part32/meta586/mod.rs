//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1970;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1971;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1972;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1973;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1974;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta586(t5979: f64, t7286: f64, t7285: f64, t5975: f64, t27820: f64, t8002: f64, t1238: f64, t24589: f64, t27808: f64, t27818: f64, t29795: f64, t29798: f64, t29804: f64, t29809: f64, t5055: f64, t6268: f64, t7283: f64, t7351: f64, t8088: f64, t1761: f64, t19232: f64, t19234: f64, t19249: f64, t2155: f64, t24587: f64, t27401: f64, t27406: f64, t27830: f64, t29667: f64, t29699: f64, t4945: f64, t8006: f64, t8015: f64, t8061: f64, t265: f64, t504: f64, t1256: f64, t1763: f64, t193: f64, t24909: f64, t27838: f64, t28755: f64, t336: f64, t4700: f64, t6270: f64, t6274: f64, t7398: f64, t28: f64, t1409: f64, t2161: f64, t28802: f64, t52: f64, t5398: f64, t8097: f64, t29514: f64, t2165: f64, t5493: f64, t113: f64, t1442: f64, t1774: f64, t28815: f64, t28819: f64, t28822: f64, t28825: f64, t28829: f64, t28833: f64, t28837: f64, t28841: f64, t28843: f64, t28861: f64, t28863: f64, t28866: f64, t29493: f64, t4028: f64, t510: f64, t5450: f64, t5457: f64, t652: f64, t7983: f64, t7989: f64, t8103: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t29506: f64, t3: f64, t1458: f64, t24972: f64, t27921: f64, t28888: f64, t28890: f64, t28892: f64, t28895: f64, t28898: f64, t28901: f64, t28903: f64, t5456: f64, t577: f64, t7423: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29812, t29813, t29816, t29817, t29822, t29825) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1970(t5979, t7286, t7285, t5975, t27820, t8002, t1238, t24589, t27808, t27818, t29795, t29798, t29804, t29809, t5055, t6268, t7283, t7351, t8088);
        let t29827 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1971(t1761, t19232, t19234, t19249, t2155, t24587, t27401, t27406, t27830, t29667, t29699, t29825, t4945, t8006, t8015, t8061, t8088);
        let t29840 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1972(t265, t504, t1256, t1763, t193, t24909, t27838, t28755, t29827, t336, t4700, t6270, t6274, t7398);
        let (t29848, t29855, t29864) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1973(t28, t1409, t2161, t28802, t29840, t52, t5398, t8097, t29514, t2165, t5493, t113, t1442, t1774, t28815, t28819, t28822, t28825, t28829, t28833, t28837, t28841, t28843, t28861, t28863, t28866, t29493, t4028, t510, t5450, t5457, t652, t7983, t7989, t8103, dens_threshold, rho1, zeta_threshold);
        let (t29865, t29866, t29884) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1974(t29506, t29864, t3, t1458, t24972, t27921, t28888, t28890, t28892, t28895, t28898, t28901, t28903, t5456, t5493, t577, t7423);
    (t29812, t29813, t29816, t29817, t29822, t29827, t29840, t29848, t29855, t29865, t29866, t29884)
}
