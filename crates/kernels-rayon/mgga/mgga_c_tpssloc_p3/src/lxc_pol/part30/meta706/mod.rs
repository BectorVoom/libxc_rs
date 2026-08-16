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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta706(t1530: f64, t16662: f64, t17109: f64, t1877: f64, t1915: f64, t23290: f64, t23295: f64, t2522: f64, t25358: f64, t25374: f64, t28448: f64, t28732: f64, t4119: f64, t4303: f64, t4314: f64, t46341: f64, t5527: f64, t5660: f64, t5664: f64, t6666: f64, t6670: f64, t67123: f64, t67164: f64, t7541: f64, t776: f64, t81539: f64, t868: f64, t86836: f64, t87975: f64, t98030: f64, t98054: f64, t98102: f64, t25: f64, t265: f64, t394: f64, t100578: f64, t100528: f64, t1409: f64, t16558: f64, t1965: f64, t25883: f64, t28756: f64, t3966: f64, t40: f64, t5398: f64, t607: f64, t6835: f64, t7643: f64, t99069: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t23788: f64, t67128: f64, t16949: f64, t25891: f64, t25927: f64, t5966: f64, t1649: f64, t4255: f64, t870: f64, t28248: f64, t83555: f64, t98011: f64, t1081: f64, t22959: f64, t25013: f64, t25354: f64, t25372: f64, t25892: f64, t25921: f64, t28771: f64, t81483: f64, t86736: f64, t97972: f64, t99064: f64, t89953: f64, t97999: f64, t10143: f64, t16944: f64, t98111: f64, t18196: f64, t25898: f64, t25945: f64, t28: f64, t28778: f64, t28789: f64, t6848: f64, t98071: f64, t99043: f64, t5544: f64, t25901: f64, t25905: f64, t25928: f64, t25938: f64, t28764: f64, t28765: f64, t6841: f64, t98027: f64, t16596: f64, t89992: f64, t98007: f64, t25365: f64, t98058: f64, t98003: f64, t25930: f64, t25934: f64, t28774: f64, t28792: f64, t28795: f64, t7649: f64, t7656: f64, t99055: f64, t504: f64, t1972: f64, t25950: f64, t28803: f64, t52: f64, t6856: f64, t7664: f64, rho1: f64, t113: f64, t20100: f64, t20136: f64, t510: f64, t6517: f64, t96654: f64, t97910: f64, t97914: f64, t97916: f64, t97919: f64, t97923: f64, t97925: f64, t97928: f64, t97930: f64, t97932: f64, t97935: f64, t97937: f64, t97941: f64, t97942: f64, t97947: f64, t97949: f64, t24987: f64, t7754: f64, t1983: f64, t2019: f64, t57806: f64, t25971: f64, t91655: f64, t26161: f64, t26162: f64, t75210: f64, t25994: f64, t7458: f64, t28817: f64, t6876: f64, t28826: f64, t83859: f64, t26149: f64, t7685: f64, t1458: f64, t1459: f64, t19461: f64, t19534: f64, t1976: f64, t2314: f64, t24980: f64, t25958: f64, t28855: f64, t4026: f64, t4034: f64, t5107: f64, t5457: f64, t6468: f64, t652: f64, t6862: f64, t6872: f64, t7451: f64, t7670: f64, t90400: f64) -> (f64, f64) {
        let t100623 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2320(t1530, t16662, t17109, t1877, t1915, t23290, t23295, t2522, t25358, t25374, t28448, t28732, t4119, t4303, t4314, t46341, t5527, t5660, t5664, t6666, t6670, t67123, t67164, t7541, t776, t81539, t868, t86836, t87975, t98030, t98054, t98102);
        let (t100624, t100637) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2321(t25, t265, t394, t100578, t100623, t100528, t1409, t16558, t1965, t25883, t28756, t3966, t40, t5398, t607, t6835, t7643, t99069, dens_threshold, rho0, zeta_threshold);
        let (t100638, t100641, t100644, t100646, t100651, t100656) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2322(t23788, t67128, t16949, t25891, t25927, t98102, t5966, t868, t1649, t4255, t870, t28248, t83555);
        let t100674 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2323(t25927, t98030, t23788, t98011, t1081, t5664, t100638, t100641, t100644, t100646, t100651, t100656, t1649, t1877, t22959, t23295, t25013, t25354, t25372, t25892, t25921, t28771, t6670, t81483, t86736, t97972, t99064);
        let (t100682, t100689, t100692, t100696, t100705, t100708) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2324(t89953, t97999, t10143, t1649, t25374, t5966, t776, t4303, t23788, t67164, t16944, t25891);
        let t100716 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2325(t25927, t98111, t100682, t100689, t100692, t100696, t100705, t100708, t18196, t1877, t1915, t22959, t25013, t2522, t25358, t25372, t25898, t25945, t28, t28778, t28789, t6666, t6670, t6848, t81539, t86736, t98054, t98071, t99043);
        let t100763 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2326(t1649, t4119, t23788, t67123, t1081, t5660, t5544, t16662, t28, t5527, t1877, t1915, t22959, t2522, t25901, t25905, t25928, t25938, t28448, t28764, t28765, t4314, t46341, t5966, t6666, t6670, t6841, t7541, t98027);
        let t100803 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2327(t16596, t89992, t23788, t98007, t17109, t28, t25365, t98058, t25927, t98003, t1081, t1877, t22959, t23290, t25013, t2522, t25354, t25358, t25930, t25934, t28448, t28774, t28792, t28795, t6666, t6670, t7649, t7656, t86836, t99055);
        let t100818 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2328(t28, t265, t504, t100674, t100716, t100763, t100803, t100624, t1409, t16558, t1972, t25950, t28803, t3966, t52, t5398, t607, t6856, t7664, dens_threshold, rho1, zeta_threshold);
        let t100822 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2329(t100637, t100818, t113, t20100, t20136, t510, t6517, t96654, t97910, t97914, t97916, t97919, t97923, t97925, t97928, t97930, t97932, t97935, t97937, t97941, t97942, t97947, t97949);
        let (t100828, t100833, t100835, t100838, t100840) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2330(t24987, t7754, t1983, t2019, t57806, t25971, t91655, t26161, t26162, t75210, t25994, t7458);
        let t100864 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2331(t28817, t6876, t1983, t28826, t83859, t26149, t7685, t100828, t100833, t100835, t100838, t100840, t1458, t1459, t19461, t19534, t1976, t2314, t24980, t25958, t28855, t4026, t4034, t5107, t5457, t6468, t652, t6862, t6872, t7451, t7458, t7670, t90400);
    (t100822, t100864)
}
