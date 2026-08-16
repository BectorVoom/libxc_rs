//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta101 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk564;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk565;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta101(t1932: f64, t475: f64, t500: f64, t526: f64, t528: f64, t118: f64, t521: f64, t2375: f64, t1294: f64, t2371: f64, t2528: f64, t2535: f64, t570: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3625, t3639, t3640, t3664, t3672, t3684) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk564(t1932, t475, t500, t526, t528, t118, t521);
        let (t3686, t3688, t3690, t3695, t3700, t3701) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk565(t2375, t3684, t1294, t2371, t2528, t2535, t570);
    (t3625, t3639, t3640, t3664, t3672, t3684, t3686, t3688, t3690, t3695, t3700, t3701)
}
