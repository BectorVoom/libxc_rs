//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta101 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk647;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk648;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta101(t240: f64, t59: f64, t40: f64, t632: f64, t73: f64, t52: f64, t636: f64, t76: f64, t111: f64, t649: f64, t107: f64, t626: f64, t667: f64, t106: f64, t655: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2281, t2282, t2289, t2291, t2296, t2298, t2314) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk647(t240, t59, t40, t632, t73, t52, t636, t76, t111, t649);
        let (t2327, t2328, t2331) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk648(t107, t2281, t626, t667, t106, t655);
    (t2281, t2282, t2289, t2291, t2296, t2298, t2314, t2327, t2328, t2331)
}
