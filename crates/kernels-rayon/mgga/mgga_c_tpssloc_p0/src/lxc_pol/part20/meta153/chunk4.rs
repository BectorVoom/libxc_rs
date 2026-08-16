//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 977/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk977(t1238: f64, t1252: f64, t3482: f64, t3484: f64, t3487: f64, t3591: f64, t3593: f64, t3600: f64, t3631: f64, t498: f64) -> f64 {
    let t3633 = 2.0_f64 * t1238 * t3600 - t1238 * t3631 - 2.0_f64 * t1252 * t3487 - 2.0_f64 * t1252 * t3593 + t3482 * t498 + 2.0_f64 * t3484 * t498 + t3591 * t498;
    t3633
}
