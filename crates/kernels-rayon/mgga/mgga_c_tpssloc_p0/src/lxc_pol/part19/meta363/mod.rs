//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta363 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1323;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1324;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1325;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta363(t1012: f64, t1015: f64, t1017: f64, t10444: f64, t41620: f64, t41622: f64, t41625: f64, t41627: f64, t41635: f64, t41639: f64, t41722: f64, t41726: f64, t41728: f64, t41732: f64, t41737: f64, t10526: f64, t2940: f64, t10623: f64, t2948: f64, t10709: f64, t2944: f64, t10632: f64, t2924: f64, t10629: f64, t2906: f64, t959: f64, t10523: f64, t10723: f64, t41804: f64, t41813: f64, t42273: f64, t42276: f64, t42280: f64, t42283: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42658, t42661) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1323(t1012, t1015, t1017, t10444, t41620, t41622, t41625, t41627, t41635, t41639, t41722, t41726, t41728, t41732, t41737);
        let (t42663, t42665, t42667, t42669, t42674, t42678) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1324(t10526, t2940, t10623, t2948, t10709, t2944, t10632, t2924, t10629, t2906, t959, t10523, t10723);
        let t42679 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1325(t41804, t41813, t42273, t42276, t42280, t42283, t42663, t42665, t42667, t42669, t42674, t42678);
    (t42658, t42661, t42663, t42665, t42667, t42669, t42674, t42678, t42679)
}
