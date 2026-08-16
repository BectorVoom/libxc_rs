//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 339/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk339<F: Float>(t1021: F, t1774: F, t1092: F, t1016: F, t1697: F, t1710: F, t1715: F, t1751: F, t1770: F, t300: F, t979: F, t393: F) -> (F, F, F, F) {
    let t1775 = t1021 * t1774;
    let t1776 = t1092 * t1775;
    let t1778 = t1697 * t300 - F::cast_from(0.66725e-1_f64) * t979 * t1710 + t1016 + F::cast_from(0.16581944444444444444e-2_f64) * t1715 + F::cast_from(0.24872916666666666666e-2_f64) * t1751 - F::cast_from(0.24872916666666666666e-2_f64) * t1770 + F::cast_from(0.16581944444444444444e-2_f64) * t1776;
    let t1779 = t1778 * t393;
    (t1775, t1776, t1778, t1779)
}
