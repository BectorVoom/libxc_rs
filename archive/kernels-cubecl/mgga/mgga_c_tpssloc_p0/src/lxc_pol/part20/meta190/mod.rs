//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta190 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1152;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1153;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1154;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1155;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1156;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1157;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta190<F: Float>(t1615: F, t376: F, t1022: F, t3131: F, t4582: F, t1023: F, t135: F, t1606: F, t973: F, t3966: F, t998: F, t974: F, t1041: F, t1607: F, t1622: F, t2960: F, t3039: F, t3048: F, t3054: F, t3070: F, t3084: F, t3092: F, t3130: F, t4562: F, t4565: F, t4572: F, t4575: F, t4579: F, t4585: F, t4590: F, t225: F, t4552: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t4593 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1152::<F>(t1615, t376);
        let t4594 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1153::<F>(t1022, t3131);
        let (t4595, t4596) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1154::<F>(t4593, t4594, t4582);
        let (t4599, t4600) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1155::<F>(t1023, t4593, t4582);
        let (t4603, t4608, t4609, t4613) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1156::<F>(t135, t1606, t973, t3966, t998, t974, t1041, t1607, t1622, t2960, t3039, t3048, t3054, t3070, t3084, t3092, t3130, t4562, t4565, t4572, t4575, t4579, t4585, t4590, t4596, t4600);
        let (t4615, t4616) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1157::<F>(t225, t4552, t68);
    (t4593, t4594, t4595, t4596, t4599, t4600, t4603, t4608, t4609, t4613, t4615, t4616)
}
