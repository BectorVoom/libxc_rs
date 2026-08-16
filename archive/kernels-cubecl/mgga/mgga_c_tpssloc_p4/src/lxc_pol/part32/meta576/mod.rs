//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta576 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1951;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1952;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1953;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta576<F: Float>(t29473: F, t67: F, t1864: F, t7445: F, t7974: F, t2109: F, t27956: F, t1860: F, t2110: F, t24514: F, t26016: F, t27298: F, t27332: F, t27341: F, t27937: F, t27961: F, t27966: F, t27972: F, t27976: F, t27979: F, t27982: F, t7246: F, t7428: F, t7432: F, t7435: F, t7975: F, t7978: F, t5: F, t112: F, t2113: F, t5456: F, t1458: F, t27863: F, t28001: F, t28004: F, t28006: F, t28009: F, t28011: F, t28019: F, t5493: F, t7266: F, t8103: F, t1459: F, t1849: F, t2114: F, t2167: F, t28027: F, t28029: F, t28032: F, t28034: F, t28036: F, t28038: F, t28040: F, t28042: F, t28047: F, t28240: F, t510: F, t5460: F, t5494: F, t574: F, t6287: F, t6468: F, t652: F, t8107: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t29474, t29475, t29478, t29481, t29484) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1951::<F>(t29473, t67, t1864, t7445, t7974, t2109, t27956, t1860, t2110, t24514, t26016, t27298, t27332, t27341, t27937, t27961, t27966, t27972, t27976, t27979, t27982, t7246, t7428, t7432, t7435, t7975, t7978);
        let (t29485, t29486, t29493, t29497) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1952::<F>(t5, t29484, t112, t2113, t5456, t1458, t27863, t28001, t28004, t28006, t28009, t28011, t28019, t5493, t7266);
        let (t29501, t29506) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1953::<F>(t1458, t8103, t1459, t1849, t2114, t2167, t27863, t28027, t28029, t28032, t28034, t28036, t28038, t28040, t28042, t28047, t28240, t29486, t29497, t510, t5460, t5494, t574, t6287, t6468, t652, t7266, t8107);
    (t29474, t29475, t29478, t29481, t29485, t29486, t29493, t29497, t29501, t29506)
}
