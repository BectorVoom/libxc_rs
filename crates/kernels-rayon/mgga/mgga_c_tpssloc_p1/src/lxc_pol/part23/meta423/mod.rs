//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta423 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1250;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1251;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta423(t1057: f64, t69923: f64, t1615: f64, t883: f64, t5866: f64, t17906: f64, t4644: f64, t17607: f64, t4571: f64, t1011: f64, t1019: f64, t1040: f64, t21482: f64, t10876: f64, t21396: f64, t248: f64, t3101: f64, t1041: f64, t21138: f64, t3051: f64, t21134: f64, t14508: f64, t17667: f64, t17611: f64, t4641: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t69924, t70100, t70122, t70132, t70138, t70148, t70153) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1250(t1057, t69923, t1615, t883, t5866, t17906, t4644, t17607, t4571, t1011, t1019, t1040, t21482);
        let (t70162, t70166, t70199, t70209, t70214) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1251(t10876, t21396, t248, t3101, t1041, t21138, t3051, t21134, t14508, t17667, t17611, t4641);
    (t69924, t70100, t70122, t70132, t70138, t70148, t70153, t70162, t70166, t70199, t70209, t70214)
}
