//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1965;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta586(t16752: f64, t252: f64, t5527: f64, t828: f64, t5611: f64, t5584: f64, t9975: f64, t852: f64, t17100: f64, t225: f64, t17087: f64, t17060: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58262, t58557, t58569, t58688, t58853, t59331, t59466, t59498, t59503) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1965(t16752, t252, t5527, t828, t5611, t5584, t9975, t852, t17100, t225, t17087, t17060);
    (t58262, t58557, t58569, t58688, t58853, t59331, t59466, t59498, t59503)
}
