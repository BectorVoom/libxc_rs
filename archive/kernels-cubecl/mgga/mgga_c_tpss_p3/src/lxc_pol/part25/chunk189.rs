//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 189/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk189<F: Float>(t600: F, t44: F, t49: F, t56: F, t589: F, t592: F, t595: F, t38: F, t45: F) -> (F, F, F, F) {
    let t601 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t600;
    let t602 = -F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t589 * t49 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t44 * t592 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t56 * t595 + t601;
    let t603 = t38 * t602;
    let t606 = t45 * t45;
    (t601, t602, t603, t606)
}
