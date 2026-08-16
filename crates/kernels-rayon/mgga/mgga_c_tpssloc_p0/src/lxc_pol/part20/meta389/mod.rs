//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta389 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1762;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1763;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1764;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1765;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1766;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta389(t13048: f64, t13470: f64, t12910: f64, t12914: f64, t12915: f64, t12922: f64, t12926: f64, t12927: f64, t12928: f64, t12934: f64, t12935: f64, t12942: f64, t12944: f64, t12947: f64, t12971: f64, t1484: f64, t1877: f64, t193: f64, t202: f64, t2522: f64, t2523: f64, t2745: f64, t2749: f64, t4255: f64, t4307: f64, t4314: f64, t766: f64, t870: f64, t9470: f64, t9724: f64, t9780: f64, t9863: f64, t10126: f64, t13095: f64, t13096: f64, t13098: f64, t13102: f64, t13103: f64, t13105: f64, t13106: f64, t13108: f64, t4119: f64, t9789: f64, t9793: f64, t9797: f64, t9820: f64, t9824: f64, t9876: f64, t9884: f64, t9887: f64, t9890: f64, t776: f64, t868: f64, t13110: f64, t13112: f64, t13114: f64, t13117: f64, t13118: f64, t13121: f64, t13122: f64, t13125: f64, t13129: f64, t13132: f64, t13135: f64, t13136: f64, t13137: f64, t2379: f64, t4310: f64, t9853: f64, t9859: f64, t9894: f64, t9907: f64, t9921: f64, t12907: f64, t2: f64, t873: f64, t584: f64, t265: f64, t16: f64, t4331: f64, t10723: f64, t4496: f64, t959: f64, t2944: f64, t4483: f64, t2940: f64, t4493: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13471, t13475) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1762(t13048, t13470, t12910, t12914, t12915, t12922, t12926, t12927, t12928, t12934, t12935, t12942, t12944, t12947, t12971, t1484, t1877, t193, t202, t2522, t2523, t2745, t2749, t4255, t4307, t4314, t766, t870, t9470, t9724, t9780, t9863);
        let t13483 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1763(t10126, t13095, t13096, t13098, t13102, t13103, t13105, t13106, t13108, t1484, t2522, t2523, t4119, t9789, t9793, t9797, t9820, t9824, t9876, t9884, t9887, t9890);
        let (t13487, t13491) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1764(t776, t868, t13110, t13112, t13114, t13117, t13118, t13121, t13122, t13125, t13129, t13132, t13135, t13136, t13137, t2379, t2522, t4307, t4310, t4314, t9853, t9859, t9894, t9907, t9921);
        let t13493 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1765(t12907, t13475, t13483, t13491);
        let (t13501, t13503, t13504, t13506, t13508, t13510, t13512, t13514) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1766(t2, t873, t584, t265, t16, t4331, t10723, t4496, t959, t2944, t4483, t2940, t4493);
    (t13471, t13487, t13493, t13501, t13503, t13504, t13506, t13508, t13510, t13512, t13514)
}
