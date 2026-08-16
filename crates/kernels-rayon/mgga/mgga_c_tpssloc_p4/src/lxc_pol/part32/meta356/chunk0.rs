//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1405/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1405(t4734: f64, t690: f64, t4778: f64, t699: f64, t4725: f64) -> (f64, f64, f64, f64, f64) {
    let t14704 = t690 * t4734;
    let t14705 = 0.20128333333333333334e0_f64 * t14704;
    let t14710 = t699 * t4778;
    let t14711 = 0.11038e0_f64 * t14710;
    let t14720 = t690 * t4725;
    (t14704, t14705, t14710, t14711, t14720)
}
