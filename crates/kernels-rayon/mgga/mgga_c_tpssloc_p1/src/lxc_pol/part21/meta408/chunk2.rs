//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1905/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1905(t4778: f64, t699: f64, t1113: f64, t14706: f64, t136: f64, t4725: f64, t690: f64) -> (f64, f64, f64, f64, f64) {
    let t14710 = t699 * t4778;
    let t14711 = 0.11038e0_f64 * t14710;
    let t14712 = t1113 * t14706;
    let t14713 = t136 * t14712;
    let t14720 = t690 * t4725;
    (t14710, t14711, t14712, t14713, t14720)
}
