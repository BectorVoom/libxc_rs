//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta237 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1407;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1408;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1409;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1410;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1411;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1412;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta237(t1021: f64, t248: f64, t5867: f64, t1615: f64, t3131: f64, t360: f64) -> (f64, f64, f64, f64, f64, f64) {
        let t5869 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1407(t1021, t248, t5867);
        let t5872 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1408(t1615);
        let t5873 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1409(t3131, t5872);
        let t5875 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1410(t1021, t248, t5873);
        let t5878 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1411(t360, t5872);
        let t5880 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1412(t1021, t248, t5878);
    (t5869, t5872, t5873, t5875, t5878, t5880)
}
