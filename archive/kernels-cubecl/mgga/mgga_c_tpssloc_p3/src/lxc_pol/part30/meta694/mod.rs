//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta694 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2214;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2215;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2216;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2217;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2218;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta694<F: Float>(t23204: F, t28298: F, t81640: F, t225: F, t28442: F, t22986: F, t23270: F, t25191: F, t4300: F, t25192: F, t86873: F, t5544: F, t857: F, t865: F, t1528: F, t2597: F, t28311: F, t866: F, t86951: F, t86968: F, t86988: F, t92432: F, t98234: F, t1527: F, t86849: F, t4272: F, t86969: F, t1520: F, t254: F, t25038: F, t25039: F, t4119: F, t1880: F, t7488: F, t87782: F, t10110: F, t17056: F, t25168: F, t25169: F, t25233: F, t25330: F, t259: F, t2713: F, t28317: F, t4142: F, t4147: F, t4268: F, t5636: F, t6662: F, t7510: F, t82120: F, t82123: F, t855: F, t92458: F, t23237: F, t28276: F, t6552: F, t16662: F, t6553: F, t6554: F, t23164: F, t16968: F, t87052: F, t87053: F, t16887: F, t87057: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t98237, t98239, t98248, t98251, t98253) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2214::<F>(t23204, t28298, t81640, t225, t28442, t22986, t23270, t25191, t4300, t25192, t86873, t5544, t857);
        let t98258 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2215::<F>(t22986, t23270, t865, t98253, t1528, t2597, t28311, t866, t86951, t86968, t86988, t92432, t98234, t98237, t98239, t98248, t98251);
        let (t98264, t98277, t98279, t98291, t98305) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2216::<F>(t1527, t22986, t23270, t86849, t4272, t86969, t1520, t254, t25038, t25039, t4119, t1880, t7488, t87782);
        let t98309 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2217::<F>(t10110, t17056, t25168, t25169, t25233, t25330, t259, t2713, t28317, t4142, t4147, t4268, t5636, t6662, t7510, t82120, t82123, t855, t92458, t98291, t98305);
        let (t98315, t98319, t98322, t98325, t98328) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2218::<F>(t23237, t28276, t6552, t16662, t6553, t6554, t23164, t23204, t16968, t87052, t87053, t16887, t87057);
    (t98258, t98264, t98277, t98279, t98309, t98315, t98319, t98322, t98325, t98328)
}
