//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 448/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk448(t1761: f64, t345: f64, t1100: f64, t1102: f64, t1697: f64, t1754: f64, t1758: f64, t278: f64, t344: f64) -> (f64, f64) {
    let t1762 = t345 * t1761;
    let t1767 = t1100 + 0.65704296666666666667e-3_f64 * t1102 * t1754 + 0.1478346675e-2_f64 * t344 * t1758 - 0.98556445e-3_f64 * t344 * t1762 - 4.0_f64 * t278 * t1697;
    (t1762, t1767)
}
