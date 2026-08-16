//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta216 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1013;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta216(t2826: f64, t5677: f64, t136: f64, t5681: f64, t908: f64, t5685: f64, t2810: f64, t2823: f64, t4335: f64, t4384: f64, t5679: f64, t5683: f64, t5687: f64, t5699: f64, t5706: f64, t5712: f64, t5714: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t5717, t5718, t5720, t5721, t5723, t5724, t5726) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1013(t2826, t5677, t136, t5681, t908, t5685, t2810, t2823, t4335, t4384, t5679, t5683, t5687, t5699, t5706, t5712, t5714);
    (t5717, t5718, t5720, t5721, t5723, t5724, t5726)
}
