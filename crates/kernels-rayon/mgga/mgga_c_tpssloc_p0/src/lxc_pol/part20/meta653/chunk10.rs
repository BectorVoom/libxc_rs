//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2418/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2418(t300: f64, t48786: f64, t48861: f64, t49076: f64, t49113: f64, t49266: f64, t49409: f64, t49450: f64, t49492: f64, t41769: f64, t4496: f64, t959: f64) -> (f64, f64) {
    let t49496 = t300 * (t48786 + t48861 + t49076 + t49113 + t49266 + t49409 + t49450 + t49492);
    let t49499 = 0.17315859105681463759e2_f64 * t959 * t4496 * t41769;
    (t49496, t49499)
}
