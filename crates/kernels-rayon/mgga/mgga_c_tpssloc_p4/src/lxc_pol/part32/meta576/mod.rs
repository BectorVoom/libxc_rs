//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta576 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1951;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1952;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1953;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta576(t29473: f64, t67: f64, t1864: f64, t7445: f64, t7974: f64, t2109: f64, t27956: f64, t1860: f64, t2110: f64, t24514: f64, t26016: f64, t27298: f64, t27332: f64, t27341: f64, t27937: f64, t27961: f64, t27966: f64, t27972: f64, t27976: f64, t27979: f64, t27982: f64, t7246: f64, t7428: f64, t7432: f64, t7435: f64, t7975: f64, t7978: f64, t5: f64, t112: f64, t2113: f64, t5456: f64, t1458: f64, t27863: f64, t28001: f64, t28004: f64, t28006: f64, t28009: f64, t28011: f64, t28019: f64, t5493: f64, t7266: f64, t8103: f64, t1459: f64, t1849: f64, t2114: f64, t2167: f64, t28027: f64, t28029: f64, t28032: f64, t28034: f64, t28036: f64, t28038: f64, t28040: f64, t28042: f64, t28047: f64, t28240: f64, t510: f64, t5460: f64, t5494: f64, t574: f64, t6287: f64, t6468: f64, t652: f64, t8107: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29474, t29475, t29478, t29481, t29484) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1951(t29473, t67, t1864, t7445, t7974, t2109, t27956, t1860, t2110, t24514, t26016, t27298, t27332, t27341, t27937, t27961, t27966, t27972, t27976, t27979, t27982, t7246, t7428, t7432, t7435, t7975, t7978);
        let (t29485, t29486, t29493, t29497) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1952(t5, t29484, t112, t2113, t5456, t1458, t27863, t28001, t28004, t28006, t28009, t28011, t28019, t5493, t7266);
        let (t29501, t29506) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1953(t1458, t8103, t1459, t1849, t2114, t2167, t27863, t28027, t28029, t28032, t28034, t28036, t28038, t28040, t28042, t28047, t28240, t29486, t29497, t510, t5460, t5494, t574, t6287, t6468, t652, t7266, t8107);
    (t29474, t29475, t29478, t29481, t29485, t29486, t29493, t29497, t29501, t29506)
}
