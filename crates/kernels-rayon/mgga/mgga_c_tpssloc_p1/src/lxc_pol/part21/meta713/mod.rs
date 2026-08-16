//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta713 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2550;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2551;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta713(t13965: f64, t3109: f64, t1041: f64, t13969: f64, t14173: f64, t247: f64, t677: f64, t4589: f64, t10969: f64, t41687: f64, t1009: f64, t13939: f64, t1011: f64, t1019: f64, t10868: f64, t248: f64, t4347: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49831, t49846, t49850) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2550(t13965, t3109, t1041, t13969, t14173, t247, t677);
        let (t49852, t49854, t49864, t49866, t49871) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2551(t1041, t4589, t49850, t10969, t41687, t1009, t13939, t1011, t1019, t10868, t248, t4347);
    (t49831, t49846, t49850, t49852, t49854, t49864, t49866, t49871)
}
