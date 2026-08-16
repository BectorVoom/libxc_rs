//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 716/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk716(t2240: f64, t7245: f64, t50: f64, t55: f64, t607: f64, t6503: f64) -> (f64, f64, f64) {
    let t7246 = t2240 * t7245;
    let t7251 = t50 * t55;
    let t7254 = -5.0_f64 / 6.0_f64 * t7251 * t607 + t6503;
    (t7246, t7251, t7254)
}
