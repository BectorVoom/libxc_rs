//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta126 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk690;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta126(t422: f64, t3236: f64, t1124: f64, t1128: f64, t1127: f64, t432: f64, t427: f64, t3293: f64, t435: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3314, t3315, t3319, t3327, t3331, t3332, t3339, t3346, t3355, t3356, t3357, t3358) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk690(t422, t3236, t1124, t1128, t1127, t432, t427, t3293, t435);
    (t3314, t3315, t3319, t3327, t3331, t3332, t3339, t3346, t3355, t3356, t3357, t3358)
}
