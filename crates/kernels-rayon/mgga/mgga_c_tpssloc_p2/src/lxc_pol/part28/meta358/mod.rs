//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta358 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1336;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1337;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1338;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1339;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1340;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1341;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1342;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta358(t2684: f64, t4295: f64, t13171: f64, t860: f64, t4265: f64, t814: f64, t829: f64, t13377: f64, t235: f64, t2679: f64, t4282: f64, t4280: f64, t808: f64, t13384: f64, t13176: f64, t13336: f64, t1499: f64, t1523: f64, t1525: f64, t226: f64, t255: f64, t2613: f64, t2617: f64, t2738: f64, t2740: f64, t4162: f64, t4166: f64, t4283: f64, t4286: f64, t4288: f64, t4291: f64, t4298: f64, t812: f64, t861: f64, t863: f64, t9612: f64, t13425: f64, t858: f64, t225: f64, t4149: f64, t13050: f64, t13053: f64, t13059: f64, t13062: f64, t13065: f64, t13068: f64, t13072: f64, t13378: f64, t259: f64, t2597: f64, t2713: f64, t2720: f64, t4268: f64, t4273: f64, t4301: f64, t855: f64, t866: f64, t13048: f64, t12910: f64, t12914: f64, t12915: f64, t12922: f64, t12926: f64, t12927: f64, t12928: f64, t12934: f64, t12935: f64, t12942: f64, t12944: f64, t12947: f64, t12971: f64, t1484: f64, t1877: f64, t193: f64, t202: f64, t2522: f64, t2523: f64, t2745: f64, t2749: f64, t4255: f64, t4307: f64, t4314: f64, t766: f64, t870: f64, t9470: f64, t9724: f64, t9780: f64, t9863: f64, t10126: f64, t13095: f64, t13096: f64, t13098: f64, t13102: f64, t13103: f64, t13105: f64, t13106: f64, t13108: f64, t4119: f64, t9789: f64, t9793: f64, t9797: f64, t9820: f64, t9824: f64, t9876: f64, t9884: f64, t9887: f64, t9890: f64, t776: f64, t868: f64, t13110: f64, t13112: f64, t13114: f64, t13117: f64, t13118: f64, t13121: f64, t13122: f64, t13125: f64, t13129: f64, t13132: f64, t13135: f64, t13136: f64, t13137: f64, t2379: f64, t4310: f64, t9853: f64, t9859: f64, t9894: f64, t9907: f64, t9921: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13429, t13431, t13434, t13448, t13450, t13453) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1336(t2684, t4295, t13171, t860, t4265, t814, t829, t13377, t235, t2679, t4282, t4280, t808);
        let (t13456, t13459) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1337(t13384, t829, t13176, t13336, t13429, t13431, t13434, t13448, t13450, t13453, t1499, t1523, t1525, t226, t255, t2613, t2617, t2738, t2740, t4162, t4166, t4283, t4286, t4288, t4291, t4298, t808, t812, t861, t863, t9612);
        let (t13460, t13461, t13463, t13470) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1338(t13425, t13459, t858, t225, t4149, t13050, t13053, t13059, t13062, t13065, t13068, t13072, t13378, t259, t2597, t2713, t2720, t4268, t4273, t4301, t855, t866);
        let (t13471, t13475) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1339(t13048, t13470, t12910, t12914, t12915, t12922, t12926, t12927, t12928, t12934, t12935, t12942, t12944, t12947, t12971, t1484, t1877, t193, t202, t2522, t2523, t2745, t2749, t4255, t4307, t4314, t766, t870, t9470, t9724, t9780, t9863);
        let t13483 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1340(t10126, t13095, t13096, t13098, t13102, t13103, t13105, t13106, t13108, t1484, t2522, t2523, t4119, t9789, t9793, t9797, t9820, t9824, t9876, t9884, t9887, t9890);
        let t13487 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1341(t776, t868);
        let t13491 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1342(t13110, t13112, t13114, t13117, t13118, t13121, t13122, t13125, t13129, t13132, t13135, t13136, t13137, t13487, t2379, t2522, t4307, t4310, t4314, t9853, t9859, t9894, t9907, t9921);
    (t13450, t13453, t13456, t13460, t13461, t13463, t13471, t13475, t13483, t13487, t13491)
}
