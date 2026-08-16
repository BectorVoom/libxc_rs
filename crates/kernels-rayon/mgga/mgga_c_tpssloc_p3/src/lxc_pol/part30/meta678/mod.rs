//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta678 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2120;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2121;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2122;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2123;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2124;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2125;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2126;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta678(t1873: f64, t96709: f64, t5464: f64, t81442: f64, t666: f64, t81446: f64, t1453: f64, t4067: f64, t22473: f64, t22470: f64, t5488: f64, t19529: f64, t6530: f64, t109: f64, t81438: f64, t81440: f64, t86589: f64, t86591: f64, t92121: f64, t1268: f64, t1458: f64, t19534: f64, t22461: f64, t24999: f64, t26103: f64, t33085: f64, t4072: f64, t5493: f64, t6517: f64, t671: f64, t90400: f64, t96361: f64, t96685: f64, t96686: f64, t96704: f64, t96706: f64, t96708: f64, t28030: f64, t6535: f64, t26114: f64, t7461: f64, t19994: f64, t24995: f64, t8945: f64, t1266: f64, t1393: f64, t1459: f64, t1774: f64, t1849: f64, t19450: f64, t19451: f64, t1976: f64, t20127: f64, t26098: f64, t26138: f64, t27993: f64, t28020: f64, t4037: f64, t4077: f64, t5494: f64, t574: f64, t652: f64, t6539: f64, t7670: f64, t96355: f64, t96358: f64, t96360: f64, t96682: f64, t28831: f64, t83886: f64, t6287: f64, t6534: f64, t26168: f64, t7685: f64, t19924: f64, t19456: f64, t7468: f64, t26003: f64, t4028: f64, t2314: f64, t28864: f64, t4034: f64, t19289: f64, t1983: f64, t20085: f64, t6996: f64, t20109: f64, t20143: f64, t24980: f64, t28852: f64, t5460: f64, t6862: f64, t28827: f64, t6876: f64, t7684: f64, t8944: f64, t26164: f64, t75203: f64, t8643: f64, t34999: f64, t5308: f64, t28813: f64, t19577: f64, t22574: f64, t33136: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96711, t96713, t96716, t96719, t96721, t96724, t96726) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2120(t1873, t96709, t5464, t81442, t666, t81446, t1453, t4067, t22473, t22470, t5488, t19529, t6530);
        let (t96729, t96731) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2121(t109, t81438, t81440, t86589, t86591, t92121, t96713, t96716, t96719, t96721, t96724, t96726, t1268);
        let t96732 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2122(t1458, t19534, t22461, t24999, t26103, t33085, t4072, t5493, t6517, t671, t90400, t96361, t96685, t96686, t96704, t96706, t96708, t96711, t96731);
        let t96749 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2123(t28030, t6535, t26114, t7461, t19994, t24995, t8945, t1266, t1393, t1459, t1774, t1849, t19450, t19451, t1976, t20127, t22461, t24999, t26098, t26138, t27993, t28020, t4037, t4072, t4077, t5494, t574, t6517, t652, t6539, t7670, t96355, t96358, t96360, t96361, t96682, t96732);
        let (t96755, t96758, t96760, t96763, t96765) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2124(t28831, t83886, t6287, t652, t6534, t26168, t7685, t19924, t24995, t8945, t19456, t7468);
        let t96793 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2125(t26003, t4028, t2314, t28864, t4034, t1873, t19289, t652, t1983, t20085, t6996, t20109, t20143, t22461, t24980, t26103, t28852, t5460, t5493, t5494, t6517, t6862, t96755, t96758, t96760, t96763, t96765);
        let (t96796, t96799, t96802, t96805, t96807, t96813) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2126(t28827, t6876, t7684, t8944, t26164, t24995, t75203, t8643, t34999, t5308, t28813, t19577, t22574, t33136);
    (t96729, t96749, t96793, t96796, t96799, t96802, t96805, t96807, t96813)
}
