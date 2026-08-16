//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta389 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1762;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1763;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1764;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1765;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1766;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta389<F: Float>(t13048: F, t13470: F, t12910: F, t12914: F, t12915: F, t12922: F, t12926: F, t12927: F, t12928: F, t12934: F, t12935: F, t12942: F, t12944: F, t12947: F, t12971: F, t1484: F, t1877: F, t193: F, t202: F, t2522: F, t2523: F, t2745: F, t2749: F, t4255: F, t4307: F, t4314: F, t766: F, t870: F, t9470: F, t9724: F, t9780: F, t9863: F, t10126: F, t13095: F, t13096: F, t13098: F, t13102: F, t13103: F, t13105: F, t13106: F, t13108: F, t4119: F, t9789: F, t9793: F, t9797: F, t9820: F, t9824: F, t9876: F, t9884: F, t9887: F, t9890: F, t776: F, t868: F, t13110: F, t13112: F, t13114: F, t13117: F, t13118: F, t13121: F, t13122: F, t13125: F, t13129: F, t13132: F, t13135: F, t13136: F, t13137: F, t2379: F, t4310: F, t9853: F, t9859: F, t9894: F, t9907: F, t9921: F, t12907: F, t2: F, t873: F, t584: F, t265: F, t16: F, t4331: F, t10723: F, t4496: F, t959: F, t2944: F, t4483: F, t2940: F, t4493: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13471, t13475) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1762::<F>(t13048, t13470, t12910, t12914, t12915, t12922, t12926, t12927, t12928, t12934, t12935, t12942, t12944, t12947, t12971, t1484, t1877, t193, t202, t2522, t2523, t2745, t2749, t4255, t4307, t4314, t766, t870, t9470, t9724, t9780, t9863);
        let t13483 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1763::<F>(t10126, t13095, t13096, t13098, t13102, t13103, t13105, t13106, t13108, t1484, t2522, t2523, t4119, t9789, t9793, t9797, t9820, t9824, t9876, t9884, t9887, t9890);
        let (t13487, t13491) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1764::<F>(t776, t868, t13110, t13112, t13114, t13117, t13118, t13121, t13122, t13125, t13129, t13132, t13135, t13136, t13137, t2379, t2522, t4307, t4310, t4314, t9853, t9859, t9894, t9907, t9921);
        let t13493 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1765::<F>(t12907, t13475, t13483, t13491);
        let (t13501, t13503, t13504, t13506, t13508, t13510, t13512, t13514) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1766::<F>(t2, t873, t584, t265, t16, t4331, t10723, t4496, t959, t2944, t4483, t2940, t4493);
    (t13471, t13487, t13493, t13501, t13503, t13504, t13506, t13508, t13510, t13512, t13514)
}
