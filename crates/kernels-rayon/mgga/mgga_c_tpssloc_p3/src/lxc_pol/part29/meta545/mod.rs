//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta545 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1942;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1943;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta545(t2122: f64, t27381: f64, t1186: f64, t4733: f64, t7286: f64, t7285: f64, t1716: f64, t24638: f64, t1760: f64, t7391: f64, t3598: f64, t24574: f64, t8003: f64, t7295: f64, t6686: f64, t8020: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27382, t27383, t27388, t27389, t27392, t27396, t27401) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1942(t2122, t27381, t1186, t4733, t7286, t7285, t1716, t24638, t1760, t7391, t3598, t24574, t8003);
        let (t27403, t27406) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1943(t1716, t7295, t6686, t8020);
    (t27382, t27383, t27388, t27389, t27392, t27396, t27401, t27403, t27406)
}
