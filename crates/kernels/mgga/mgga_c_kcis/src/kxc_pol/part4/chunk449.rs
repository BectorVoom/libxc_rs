//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 449/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk449<F: Float>(t1131: F, t1773: F, t1021: F, t1092: F, t1016: F, t1697: F, t1710: F, t1715: F, t1751: F, t1770: F, t300: F, t979: F) -> (F, F, F, F) {
    let t1774 = t1131 * t1773;
    let t1775 = t1021 * t1774;
    let t1776 = t1092 * t1775;
    let t1778 = t1697 * t300 - F::new(0.66725e-1) * t979 * t1710 + t1016 + F::new(0.16581944444444444444e-2) * t1715 + F::new(0.24872916666666666666e-2) * t1751 - F::new(0.24872916666666666666e-2) * t1770 + F::new(0.16581944444444444444e-2) * t1776;
    (t1774, t1775, t1776, t1778)
}
