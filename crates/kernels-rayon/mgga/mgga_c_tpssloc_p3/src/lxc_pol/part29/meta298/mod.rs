//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta298 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1339;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta298(t2427: f64, t2430: f64, t32: f64, t717: f64, t2244: f64, t751: f64, t2658: f64, t813: f64, t236: f64, t232: f64, t2632: f64, t2639: f64, t2686: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t9924, t9929, t9933, t9971, t9972, t9975, t9986) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1339(t2427, t2430, t32, t717, t2244, t751, t2658, t813, t236, t232, t2632, t2639, t2686);
    (t9924, t9929, t9933, t9971, t9972, t9975, t9986)
}
