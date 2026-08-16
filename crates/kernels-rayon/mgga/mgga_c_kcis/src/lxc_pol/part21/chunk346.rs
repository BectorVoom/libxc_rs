//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 346/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk346(t1749: f64, t360: f64, t303: f64, t1103: f64, t1104: f64, t1646: f64, t1109: f64, t1670: f64, t345: f64, t1114: f64, t1727: f64, t1100: f64, t1102: f64, t1697: f64, t278: f64, t344: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1750 = t1749 * t360;
    let t1751 = t303 * t1750;
    let t1754 = t1103 * t1104 * t1646;
    let t1757 = t1109 * t1670;
    let t1758 = t345 * t1757;
    let t1761 = t1114 * t1727;
    let t1762 = t345 * t1761;
    let t1767 = t1100 + 0.65704296666666666667e-3_f64 * t1102 * t1754 + 0.1478346675e-2_f64 * t344 * t1758 - 0.98556445e-3_f64 * t344 * t1762 - 4.0_f64 * t278 * t1697;
    (t1750, t1751, t1754, t1757, t1758, t1761, t1762, t1767)
}
