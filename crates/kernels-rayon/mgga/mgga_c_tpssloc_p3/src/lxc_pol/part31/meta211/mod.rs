//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta211 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk940;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk941;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta211(t2768: f64, t5677: f64, t123: f64, t2775: f64, t5392: f64, t882: f64, t5398: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t5678, t5679, t5681) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk940(t2768, t5677, t123, t2775, t5392);
        let (t5682, t5683, t5685) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk941(t5681, t882, t123, t5398, t883);
    (t5678, t5679, t5681, t5682, t5683, t5685)
}
