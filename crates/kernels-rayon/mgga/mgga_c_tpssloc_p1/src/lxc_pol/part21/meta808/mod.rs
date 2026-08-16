//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta808 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2822;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2823;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2824;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2825;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2826;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2827;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta808(t2: f64, t4324: f64, t584: f64, t1534: f64, t16: f64, t17139: f64, t14389: f64, t48763: f64, t41656: f64, t47705: f64, t47707: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47724: f64, t47730: f64, t47732: f64, t47738: f64, t47774: f64, t47783: f64, t55716: f64, t2394: f64, t5678: f64, t17156: f64, t2244: f64, t123: f64, t882: f64, t17184: f64, t690: f64, t17179: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59627, t59629, t59631, t59637, t59650) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2822(t2, t4324, t584, t1534, t16, t17139, t14389, t48763, t41656, t47705, t47707, t47709, t47711, t47713, t47715, t47717, t47724, t47730, t47732, t47738);
        let t59655 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2823(t47774, t47783, t55716);
        let t59657 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2824(t2394, t5678);
        let (t59659, t59661) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2825(t17156, t2244, t123, t882);
        let t59663 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2826(t17184, t690);
        let t59665 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2827(t17179, t690);
    (t59627, t59629, t59631, t59637, t59650, t59655, t59657, t59659, t59661, t59663, t59665)
}
