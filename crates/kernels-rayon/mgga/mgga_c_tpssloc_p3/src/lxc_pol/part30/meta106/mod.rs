//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta106 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk661;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk662;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta106(t730: f64, t731: f64, t2388: f64, t2391: f64, t2394: f64, t2398: f64, t2400: f64, t2403: f64, t723: f64, t159: f64, t167: f64, t676: f64, t682: f64, t268: f64, t703: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2461, t2462, t2471, t2472, t2475, t2476, t2477, t2478, t2479, t2480, t2483) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk661(t730, t731, t2388, t2391, t2394, t2398, t2400, t2403, t723, t159, t167, t676, t682);
        let t2486 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk662(t2483, t268, t703);
    (t2461, t2462, t2471, t2472, t2475, t2476, t2477, t2478, t2479, t2480, t2483, t2486)
}
