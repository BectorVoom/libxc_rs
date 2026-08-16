//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta706 (260520-c91 hierarchical CSE).
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
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2320;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2321;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2322;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2323;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2324;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2325;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2326;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2327;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2328;
use chunk9::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2329;
use chunk10::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2330;
use chunk11::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2331;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta706<F: Float>(t1530: F, t16662: F, t17109: F, t1877: F, t1915: F, t23290: F, t23295: F, t2522: F, t25358: F, t25374: F, t28448: F, t28732: F, t4119: F, t4303: F, t4314: F, t46341: F, t5527: F, t5660: F, t5664: F, t6666: F, t6670: F, t67123: F, t67164: F, t7541: F, t776: F, t81539: F, t868: F, t86836: F, t87975: F, t98030: F, t98054: F, t98102: F, t25: F, t265: F, t394: F, t100578: F, t100528: F, t1409: F, t16558: F, t1965: F, t25883: F, t28756: F, t3966: F, t40: F, t5398: F, t607: F, t6835: F, t7643: F, t99069: F, dens_threshold: F, rho0: F, zeta_threshold: F, t23788: F, t67128: F, t16949: F, t25891: F, t25927: F, t5966: F, t1649: F, t4255: F, t870: F, t28248: F, t83555: F, t98011: F, t1081: F, t22959: F, t25013: F, t25354: F, t25372: F, t25892: F, t25921: F, t28771: F, t81483: F, t86736: F, t97972: F, t99064: F, t89953: F, t97999: F, t10143: F, t16944: F, t98111: F, t18196: F, t25898: F, t25945: F, t28: F, t28778: F, t28789: F, t6848: F, t98071: F, t99043: F, t5544: F, t25901: F, t25905: F, t25928: F, t25938: F, t28764: F, t28765: F, t6841: F, t98027: F, t16596: F, t89992: F, t98007: F, t25365: F, t98058: F, t98003: F, t25930: F, t25934: F, t28774: F, t28792: F, t28795: F, t7649: F, t7656: F, t99055: F, t504: F, t1972: F, t25950: F, t28803: F, t52: F, t6856: F, t7664: F, rho1: F, t113: F, t20100: F, t20136: F, t510: F, t6517: F, t96654: F, t97910: F, t97914: F, t97916: F, t97919: F, t97923: F, t97925: F, t97928: F, t97930: F, t97932: F, t97935: F, t97937: F, t97941: F, t97942: F, t97947: F, t97949: F, t24987: F, t7754: F, t1983: F, t2019: F, t57806: F, t25971: F, t91655: F, t26161: F, t26162: F, t75210: F, t25994: F, t7458: F, t28817: F, t6876: F, t28826: F, t83859: F, t26149: F, t7685: F, t1458: F, t1459: F, t19461: F, t19534: F, t1976: F, t2314: F, t24980: F, t25958: F, t28855: F, t4026: F, t4034: F, t5107: F, t5457: F, t6468: F, t652: F, t6862: F, t6872: F, t7451: F, t7670: F, t90400: F) -> (F, F) {
        let t100623 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2320::<F>(t1530, t16662, t17109, t1877, t1915, t23290, t23295, t2522, t25358, t25374, t28448, t28732, t4119, t4303, t4314, t46341, t5527, t5660, t5664, t6666, t6670, t67123, t67164, t7541, t776, t81539, t868, t86836, t87975, t98030, t98054, t98102);
        let (t100624, t100637) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2321::<F>(t25, t265, t394, t100578, t100623, t100528, t1409, t16558, t1965, t25883, t28756, t3966, t40, t5398, t607, t6835, t7643, t99069, dens_threshold, rho0, zeta_threshold);
        let (t100638, t100641, t100644, t100646, t100651, t100656) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2322::<F>(t23788, t67128, t16949, t25891, t25927, t98102, t5966, t868, t1649, t4255, t870, t28248, t83555);
        let t100674 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2323::<F>(t25927, t98030, t23788, t98011, t1081, t5664, t100638, t100641, t100644, t100646, t100651, t100656, t1649, t1877, t22959, t23295, t25013, t25354, t25372, t25892, t25921, t28771, t6670, t81483, t86736, t97972, t99064);
        let (t100682, t100689, t100692, t100696, t100705, t100708) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2324::<F>(t89953, t97999, t10143, t1649, t25374, t5966, t776, t4303, t23788, t67164, t16944, t25891);
        let t100716 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2325::<F>(t25927, t98111, t100682, t100689, t100692, t100696, t100705, t100708, t18196, t1877, t1915, t22959, t25013, t2522, t25358, t25372, t25898, t25945, t28, t28778, t28789, t6666, t6670, t6848, t81539, t86736, t98054, t98071, t99043);
        let t100763 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2326::<F>(t1649, t4119, t23788, t67123, t1081, t5660, t5544, t16662, t28, t5527, t1877, t1915, t22959, t2522, t25901, t25905, t25928, t25938, t28448, t28764, t28765, t4314, t46341, t5966, t6666, t6670, t6841, t7541, t98027);
        let t100803 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2327::<F>(t16596, t89992, t23788, t98007, t17109, t28, t25365, t98058, t25927, t98003, t1081, t1877, t22959, t23290, t25013, t2522, t25354, t25358, t25930, t25934, t28448, t28774, t28792, t28795, t6666, t6670, t7649, t7656, t86836, t99055);
        let t100818 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2328::<F>(t28, t265, t504, t100674, t100716, t100763, t100803, t100624, t1409, t16558, t1972, t25950, t28803, t3966, t52, t5398, t607, t6856, t7664, dens_threshold, rho1, zeta_threshold);
        let t100822 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2329::<F>(t100637, t100818, t113, t20100, t20136, t510, t6517, t96654, t97910, t97914, t97916, t97919, t97923, t97925, t97928, t97930, t97932, t97935, t97937, t97941, t97942, t97947, t97949);
        let (t100828, t100833, t100835, t100838, t100840) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2330::<F>(t24987, t7754, t1983, t2019, t57806, t25971, t91655, t26161, t26162, t75210, t25994, t7458);
        let t100864 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2331::<F>(t28817, t6876, t1983, t28826, t83859, t26149, t7685, t100828, t100833, t100835, t100838, t100840, t1458, t1459, t19461, t19534, t1976, t2314, t24980, t25958, t28855, t4026, t4034, t5107, t5457, t6468, t652, t6862, t6872, t7451, t7458, t7670, t90400);
    (t100822, t100864)
}
