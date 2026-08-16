//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta363 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1323;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1324;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1325;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta363<F: Float>(t1012: F, t1015: F, t1017: F, t10444: F, t41620: F, t41622: F, t41625: F, t41627: F, t41635: F, t41639: F, t41722: F, t41726: F, t41728: F, t41732: F, t41737: F, t10526: F, t2940: F, t10623: F, t2948: F, t10709: F, t2944: F, t10632: F, t2924: F, t10629: F, t2906: F, t959: F, t10523: F, t10723: F, t41804: F, t41813: F, t42273: F, t42276: F, t42280: F, t42283: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t42658, t42661) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1323::<F>(t1012, t1015, t1017, t10444, t41620, t41622, t41625, t41627, t41635, t41639, t41722, t41726, t41728, t41732, t41737);
        let (t42663, t42665, t42667, t42669, t42674, t42678) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1324::<F>(t10526, t2940, t10623, t2948, t10709, t2944, t10632, t2924, t10629, t2906, t959, t10523, t10723);
        let t42679 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1325::<F>(t41804, t41813, t42273, t42276, t42280, t42283, t42663, t42665, t42667, t42669, t42674, t42678);
    (t42658, t42661, t42663, t42665, t42667, t42669, t42674, t42678, t42679)
}
