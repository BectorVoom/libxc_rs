//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta399 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1208;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta399(t2860: f64, t5737: f64, t10813: f64, t5758: f64, t2841: f64, t5689: f64, t2403: f64, t5720: f64, t5723: f64, t5717: f64, t2929: f64, t5769: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t59920, t59941, t59959, t60168, t60173, t60204, t60343) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1208(t2860, t5737, t10813, t5758, t2841, t5689, t2403, t5720, t5723, t5717, t2929, t5769);
    (t59920, t59941, t59959, t60168, t60173, t60204, t60343)
}
