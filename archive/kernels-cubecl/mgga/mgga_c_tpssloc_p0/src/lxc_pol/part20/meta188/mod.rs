//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta188 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1142;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1143;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1144;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta188<F: Float>(t1597: F, t984: F, t343: F, t4546: F, t1593: F, t1600: F, t2958: F, t2960: F, t2969: F, t2972: F, t2975: F, t2986: F, t4507: F, t4511: F, t4515: F, t4519: F, t4523: F, t4529: F, t4532: F, t4543: F, t973: F, t381: F, t1049: F, t1603: F, t1604: F, t225: F, t1625: F, t990: F, t4343: F, t977: F, t2979: F, t4338: F, t1539: F, t248: F, t3051: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t4548, t4549, t4552) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1142::<F>(t1597, t984, t343, t4546, t1593, t1600, t2958, t2960, t2969, t2972, t2975, t2986, t4507, t4511, t4515, t4519, t4523, t4529, t4532, t4543, t973);
        let (t4553, t4555, t4557, t4559, t4562, t4565) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1143::<F>(t381, t4552, t1049, t1603, t1604, t225, t1625, t990, t4343, t977, t2979, t4338);
        let t4571 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1144::<F>(t1539, t248, t3051);
    (t4548, t4549, t4552, t4553, t4555, t4557, t4559, t4562, t4565, t4571)
}
