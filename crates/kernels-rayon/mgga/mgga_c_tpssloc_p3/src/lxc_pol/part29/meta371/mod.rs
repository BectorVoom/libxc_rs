//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1486;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta371(t12907: f64, t13475: f64, t13483: f64, t13491: f64, t2: f64, t873: f64, t584: f64, t265: f64, t16: f64, t4331: f64, t10723: f64, t4496: f64, t959: f64, t2944: f64, t4483: f64, t2940: f64, t4493: f64, t4351: f64, t892: f64, t914: f64, t2837: f64, t4354: f64, t1543: f64, t2841: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13493, t13503, t13504, t13506, t13508) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1486(t12907, t13475, t13483, t13491, t2, t873, t584, t265, t16, t4331, t10723, t4496);
        let (t13510, t13512, t13514, t13517, t13519, t13520) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1487(t13508, t959, t2944, t4483, t2940, t4493, t4351, t892, t914, t2837, t4354, t1543, t2841);
    (t13493, t13503, t13504, t13506, t13510, t13512, t13514, t13517, t13519, t13520)
}
