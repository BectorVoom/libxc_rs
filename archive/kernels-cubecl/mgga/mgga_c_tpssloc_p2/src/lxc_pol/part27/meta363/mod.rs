//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta363 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1489;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1490;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1491;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1492;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1493;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1494;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1495;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta363<F: Float>(t2684: F, t4295: F, t13171: F, t860: F, t4265: F, t814: F, t829: F, t13377: F, t235: F, t2679: F, t4282: F, t4280: F, t808: F, t13384: F, t13176: F, t13336: F, t1499: F, t1523: F, t1525: F, t226: F, t255: F, t2613: F, t2617: F, t2738: F, t2740: F, t4162: F, t4166: F, t4283: F, t4286: F, t4288: F, t4291: F, t4298: F, t812: F, t861: F, t863: F, t9612: F, t13425: F, t858: F, t225: F, t4149: F, t13050: F, t13053: F, t13059: F, t13062: F, t13065: F, t13068: F, t13072: F, t13378: F, t259: F, t2597: F, t2713: F, t2720: F, t4268: F, t4273: F, t4301: F, t855: F, t866: F, t13048: F, t12910: F, t12914: F, t12915: F, t12922: F, t12926: F, t12927: F, t12928: F, t12934: F, t12935: F, t12942: F, t12944: F, t12947: F, t12971: F, t1484: F, t1877: F, t193: F, t202: F, t2522: F, t2523: F, t2745: F, t2749: F, t4255: F, t4307: F, t4314: F, t766: F, t870: F, t9470: F, t9724: F, t9780: F, t9863: F, t10126: F, t13095: F, t13096: F, t13098: F, t13102: F, t13103: F, t13105: F, t13106: F, t13108: F, t4119: F, t9789: F, t9793: F, t9797: F, t9820: F, t9824: F, t9876: F, t9884: F, t9887: F, t9890: F, t776: F, t868: F, t13110: F, t13112: F, t13114: F, t13117: F, t13118: F, t13121: F, t13122: F, t13125: F, t13129: F, t13132: F, t13135: F, t13136: F, t13137: F, t2379: F, t4310: F, t9853: F, t9859: F, t9894: F, t9907: F, t9921: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13429, t13431, t13434, t13448, t13450, t13453) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1489::<F>(t2684, t4295, t13171, t860, t4265, t814, t829, t13377, t235, t2679, t4282, t4280, t808);
        let (t13456, t13459) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1490::<F>(t13384, t829, t13176, t13336, t13429, t13431, t13434, t13448, t13450, t13453, t1499, t1523, t1525, t226, t255, t2613, t2617, t2738, t2740, t4162, t4166, t4283, t4286, t4288, t4291, t4298, t808, t812, t861, t863, t9612);
        let (t13460, t13461, t13463, t13470) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1491::<F>(t13425, t13459, t858, t225, t4149, t13050, t13053, t13059, t13062, t13065, t13068, t13072, t13378, t259, t2597, t2713, t2720, t4268, t4273, t4301, t855, t866);
        let (t13471, t13475) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1492::<F>(t13048, t13470, t12910, t12914, t12915, t12922, t12926, t12927, t12928, t12934, t12935, t12942, t12944, t12947, t12971, t1484, t1877, t193, t202, t2522, t2523, t2745, t2749, t4255, t4307, t4314, t766, t870, t9470, t9724, t9780, t9863);
        let t13483 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1493::<F>(t10126, t13095, t13096, t13098, t13102, t13103, t13105, t13106, t13108, t1484, t2522, t2523, t4119, t9789, t9793, t9797, t9820, t9824, t9876, t9884, t9887, t9890);
        let t13487 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1494::<F>(t776, t868);
        let t13491 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1495::<F>(t13110, t13112, t13114, t13117, t13118, t13121, t13122, t13125, t13129, t13132, t13135, t13136, t13137, t13487, t2379, t2522, t4307, t4310, t4314, t9853, t9859, t9894, t9907, t9921);
    (t13450, t13453, t13456, t13460, t13461, t13463, t13471, t13475, t13483, t13487, t13491)
}
