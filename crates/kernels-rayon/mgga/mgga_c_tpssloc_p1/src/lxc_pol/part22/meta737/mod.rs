//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta737 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2420;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2421;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta737(t49486: f64, t5695: f64, t10655: f64, t21253: f64, t17521: f64, t48763: f64, t21347: f64, t300: f64, t961: f64, t10702: f64, t14395: f64, t5726: f64, t912: f64, t17366: f64, t4488: f64, t959: f64, t21091: f64, t2940: f64, t21373: f64, t17930: f64, t4483: f64, t17564: f64, t48890: f64, t1068: f64, t21376: f64, t43637: f64, t4700: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t69003, t69005, t69011, t69014, t69018) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2420(t49486, t5695, t10655, t21253, t17521, t48763, t21347, t300, t961, t10702, t14395, t5726, t912);
        let (t69021, t69023, t69025, t69027, t69030, t69031) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2421(t17366, t4488, t959, t21091, t2940, t21373, t17930, t4483, t17564, t48890, t1068, t21376, t43637, t4700, t69003, t69005, t69011, t69014, t69018);
    (t69003, t69005, t69011, t69014, t69018, t69021, t69023, t69025, t69027, t69030, t69031)
}
