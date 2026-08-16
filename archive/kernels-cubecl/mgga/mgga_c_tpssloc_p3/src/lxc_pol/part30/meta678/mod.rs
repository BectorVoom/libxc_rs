//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta678 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2120;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2121;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2122;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2123;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2124;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2125;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2126;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta678<F: Float>(t1873: F, t96709: F, t5464: F, t81442: F, t666: F, t81446: F, t1453: F, t4067: F, t22473: F, t22470: F, t5488: F, t19529: F, t6530: F, t109: F, t81438: F, t81440: F, t86589: F, t86591: F, t92121: F, t1268: F, t1458: F, t19534: F, t22461: F, t24999: F, t26103: F, t33085: F, t4072: F, t5493: F, t6517: F, t671: F, t90400: F, t96361: F, t96685: F, t96686: F, t96704: F, t96706: F, t96708: F, t28030: F, t6535: F, t26114: F, t7461: F, t19994: F, t24995: F, t8945: F, t1266: F, t1393: F, t1459: F, t1774: F, t1849: F, t19450: F, t19451: F, t1976: F, t20127: F, t26098: F, t26138: F, t27993: F, t28020: F, t4037: F, t4077: F, t5494: F, t574: F, t652: F, t6539: F, t7670: F, t96355: F, t96358: F, t96360: F, t96682: F, t28831: F, t83886: F, t6287: F, t6534: F, t26168: F, t7685: F, t19924: F, t19456: F, t7468: F, t26003: F, t4028: F, t2314: F, t28864: F, t4034: F, t19289: F, t1983: F, t20085: F, t6996: F, t20109: F, t20143: F, t24980: F, t28852: F, t5460: F, t6862: F, t28827: F, t6876: F, t7684: F, t8944: F, t26164: F, t75203: F, t8643: F, t34999: F, t5308: F, t28813: F, t19577: F, t22574: F, t33136: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t96711, t96713, t96716, t96719, t96721, t96724, t96726) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2120::<F>(t1873, t96709, t5464, t81442, t666, t81446, t1453, t4067, t22473, t22470, t5488, t19529, t6530);
        let (t96729, t96731) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2121::<F>(t109, t81438, t81440, t86589, t86591, t92121, t96713, t96716, t96719, t96721, t96724, t96726, t1268);
        let t96732 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2122::<F>(t1458, t19534, t22461, t24999, t26103, t33085, t4072, t5493, t6517, t671, t90400, t96361, t96685, t96686, t96704, t96706, t96708, t96711, t96731);
        let t96749 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2123::<F>(t28030, t6535, t26114, t7461, t19994, t24995, t8945, t1266, t1393, t1459, t1774, t1849, t19450, t19451, t1976, t20127, t22461, t24999, t26098, t26138, t27993, t28020, t4037, t4072, t4077, t5494, t574, t6517, t652, t6539, t7670, t96355, t96358, t96360, t96361, t96682, t96732);
        let (t96755, t96758, t96760, t96763, t96765) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2124::<F>(t28831, t83886, t6287, t652, t6534, t26168, t7685, t19924, t24995, t8945, t19456, t7468);
        let t96793 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2125::<F>(t26003, t4028, t2314, t28864, t4034, t1873, t19289, t652, t1983, t20085, t6996, t20109, t20143, t22461, t24980, t26103, t28852, t5460, t5493, t5494, t6517, t6862, t96755, t96758, t96760, t96763, t96765);
        let (t96796, t96799, t96802, t96805, t96807, t96813) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2126::<F>(t28827, t6876, t7684, t8944, t26164, t24995, t75203, t8643, t34999, t5308, t28813, t19577, t22574, t33136);
    (t96729, t96749, t96793, t96796, t96799, t96802, t96805, t96807, t96813)
}
