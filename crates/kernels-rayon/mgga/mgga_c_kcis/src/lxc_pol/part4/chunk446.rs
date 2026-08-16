//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 446/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk446(t1747: f64, t304: f64, t355: f64, t360: f64, t303: f64, t1103: f64, t1104: f64, t1646: f64, t1109: f64, t1670: f64, t345: f64, t1114: f64, t1727: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1748 = t304 * t1747;
    let t1749 = t1748 * t355;
    let t1750 = t1749 * t360;
    let t1751 = t303 * t1750;
    let t1754 = t1103 * t1104 * t1646;
    let t1757 = t1109 * t1670;
    let t1758 = t345 * t1757;
    let t1761 = t1114 * t1727;
    (t1749, t1750, t1751, t1754, t1757, t1758, t1761)
}
