//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta258 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1391;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1392;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1393;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1394;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1395;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta258(t10121: f64, t193: f64, t202: f64, t2379: f64, t2522: f64, t2523: f64, t2553: f64, t262: f64, t4314: f64, t766: f64, t776: f64, t870: f64, t9450: f64, t9457: f64, t9458: f64, t9463: f64, t9469: f64, t9470: f64, t9476: f64, t9484: f64, t9496: f64, t9516: f64, t2745: f64, t2752: f64, t1877: f64, t868: f64, t9684: f64, t9715: f64, t9718: f64, t9724: f64, t9727: f64, t9780: f64, t9789: f64, t9863: f64, t9865: f64, t9867: f64, t9870: f64, t2749: f64, t261: f64, t2751: f64, t9793: f64, t9797: f64, t9820: f64, t9824: f64, t9872: f64, t9876: f64, t9881: f64, t9884: f64, t9887: f64, t9890: f64, t9894: f64, t9896: f64, t9853: f64, t9859: f64, t9900: f64, t9903: f64, t9907: f64, t9911: f64, t9914: f64, t9917: f64, t9921: f64, t9923: f64, t9925: f64, t9928: f64, t9931: f64, t9934: f64) -> (f64, f64, f64, f64, f64) {
        let t10125 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1391(t10121, t193, t202, t2379, t2522, t2523, t2553, t262, t4314, t766, t776, t870, t9450, t9457, t9458, t9463, t9469, t9470, t9476, t9484, t9496, t9516);
        let (t10126, t10134, t10138) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1392(t2745, t870, t2553, t262, t2752, t1877, t2522, t4314, t776, t868, t9684, t9715, t9718, t9724, t9727, t9780, t9789, t9863, t9865, t9867, t9870);
        let (t10140, t10143, t10147) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1393(t2749, t868, t261, t2751, t193, t202, t9793, t9797, t9820, t9824, t9872, t9876, t9881, t9884, t9887, t9890, t9894, t9896);
        let t10148 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1394(t9853, t9859, t9900, t9903, t9907, t9911, t9914, t9917, t9921, t9923, t9925, t9928, t9931, t9934);
        let t10150 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1395(t10125, t10138, t10147, t10148);
    (t10126, t10134, t10140, t10143, t10150)
}
