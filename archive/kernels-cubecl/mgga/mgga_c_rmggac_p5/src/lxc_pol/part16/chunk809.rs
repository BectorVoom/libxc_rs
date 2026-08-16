//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 809/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk809<F: Float>(t36924: F, t9082: F, t321: F, t8915: F, t262: F, t7204: F, t2157: F, t5011: F, t333: F, t8708: F, t7198: F, t352: F) -> (F, F, F, F, F, F, F, F, F) {
    let t39609 = t36924 * t9082;
    let t39665 = t8915 * t321;
    let t39666 = t262 * t39665;
    let t39667 = t7204 * t39666;
    let t39678 = t5011 * t2157;
    let t39692 = t8708 * t333;
    let t39693 = t262 * t39692;
    let t39694 = t7198 * t39693;
    let t39696 = t8708 * t352;
    (t39609, t39665, t39666, t39667, t39678, t39692, t39693, t39694, t39696)
}
