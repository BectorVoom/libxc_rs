//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta451 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1299;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1300;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1301;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta451(t5664: f64, t67159: f64, t58021: f64, t46278: f64, t67177: f64, t1484: f64, t1530: f64, t1877: f64, t193: f64, t202: f64, t39483: f64, t40741: f64, t40743: f64, t40748: f64, t40760: f64, t40764: f64, t40766: f64, t40772: f64, t4314: f64, t67154: f64, t67235: f64, t67179: f64, t67185: f64, t46302: f64, t67209: f64, t16: f64, t39031: f64, t25: f64, t28: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t75884, t75885, t75886, t75887, t75891) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1299(t5664, t67159, t58021, t46278, t67177, t1484, t1530, t1877, t193, t202, t39483, t40741, t40743, t40748, t40760, t40764, t40766, t40772, t4314, t67154, t67235);
        let (t75894, t75895, t75900, t75901, t75910, t75911) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1300(t67179, t67185, t46302, t67209, t16, t39031);
        let t75912 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1301(t25, t28, t75911, zeta_threshold);
    (t75884, t75885, t75886, t75887, t75891, t75894, t75895, t75900, t75901, t75910, t75911, t75912)
}
