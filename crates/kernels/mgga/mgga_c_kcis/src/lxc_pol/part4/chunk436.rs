//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 436/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk436<F: Float>(t1747: F, t304: F, t355: F, t360: F, t303: F, t1103: F, t1104: F, t1646: F, t1109: F, t1670: F, t345: F, t1114: F, t1727: F, t1100: F, t1102: F, t1697: F, t278: F, t344: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1748 = t304 * t1747;
    let t1749 = t1748 * t355;
    let t1750 = t1749 * t360;
    let t1751 = t303 * t1750;
    let t1754 = t1103 * t1104 * t1646;
    let t1757 = t1109 * t1670;
    let t1758 = t345 * t1757;
    let t1761 = t1114 * t1727;
    let t1762 = t345 * t1761;
    let t1767 = t1100 + 0.65704296666666666667e-3 * t1102 * t1754 + 0.1478346675e-2 * t344 * t1758 - 0.98556445e-3 * t344 * t1762 - 4.0 * t278 * t1697;
    (t1749, t1750, t1751, t1754, t1757, t1758, t1761, t1762, t1767)
}
