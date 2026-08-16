//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta104 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk575;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk576;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta104(t1291: f64, t2663: f64, t2225: f64, t522: f64, t2221: f64, t2223: f64, t2516: f64, t521: f64, t17: f64, t1287: f64, t592: f64, t588: f64, t1365: f64, t68: f64, t248: f64, t2691: f64, t557: f64, t555: f64, t1361: f64, t835: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3813, t3819, t3821, t3823, t3824) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk575(t1291, t2663, t2225, t522, t2221, t2223, t2516, t521);
        let (t3825, t3832, t3836, t3843, t3862, t3864, t3865) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk576(t17, t3824, t1287, t592, t588, t1365, t68, t248, t2691, t557, t555, t1361, t835);
    (t3813, t3819, t3821, t3823, t3824, t3825, t3832, t3836, t3843, t3862, t3864, t3865)
}
