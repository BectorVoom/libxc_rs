//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta323 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1595;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1596;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta323(t11638: f64, t491: f64, t1246: f64, t1222: f64, t3567: f64, t1203: f64, t3540: f64, t2393: f64, t374: f64, t486: f64, t485: f64, t248: f64, t3516: f64, t3570: f64, t3515: f64, t11154: f64, t3585: f64, t3493: f64, t4978: f64, t4582: f64, t3576: f64, t3604: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11640, t11642, t11644, t11647, t11649, t11651) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1595(t11638, t491, t1246, t1222, t3567, t1203, t3540, t2393, t374, t486, t485, t248, t3516, t3570);
        let (t11652, t11655, t11660, t11661, t11662, t11665) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1596(t11651, t3515, t11154, t248, t3585, t3493, t486, t4978, t4582, t3576, t3604);
    (t11640, t11642, t11644, t11647, t11649, t11651, t11652, t11655, t11660, t11661, t11662, t11665)
}
