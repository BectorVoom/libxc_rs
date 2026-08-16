//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta160 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1016;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1017;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1018;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1019;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta160(t1365: f64, t67: f64, t246: f64, t120: f64, t1351: f64, t1307: f64, t550: f64, t1291: f64, t2663: f64, t1284: f64, t758: f64, t2408: f64, t2417: f64, t2426: f64, t2486: f64, t3683: f64, t3688: f64, t3690: f64, t3693: f64, t3695: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3804, t3805) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1016(t1365, t67, t246);
        let (t3806, t3807) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1017(t120, t1351, t1307, t550);
        let t3809 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1018(t3805, t3806, t3807);
        let (t3813, t3814, t3815, t3816, t3817) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1019(t1291, t2663, t1284, t67, t758, t2408, t2417, t2426, t2486, t3683, t3688, t3690, t3693, t3695);
    (t3804, t3805, t3806, t3807, t3809, t3813, t3814, t3815, t3816, t3817)
}
