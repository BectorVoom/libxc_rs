//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 332/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk332<F: Float>(t1761: F, t345: F, t1100: F, t1102: F, t1697: F, t1754: F, t1758: F, t278: F, t344: F) -> (F, F) {
    let t1762 = t345 * t1761;
    let t1767 = t1100 + 0.65704296666666666667e-3 * t1102 * t1754 + 0.1478346675e-2 * t344 * t1758 - 0.98556445e-3 * t344 * t1762 - 4.0 * t278 * t1697;
    (t1762, t1767)
}
