//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta400 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1209;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta400(t2791: f64, t5689: f64, t10704: f64, t5726: f64, t2885: f64, t5737: f64, t2904: f64, t5769: f64, t10632: f64, t5790: f64, t11094: f64, t5946: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t60357, t60378, t60407, t60424, t60722, t60874) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1209(t2791, t5689, t10704, t5726, t2885, t5737, t2904, t5769, t10632, t5790, t11094, t5946);
    (t60357, t60378, t60407, t60424, t60722, t60874)
}
