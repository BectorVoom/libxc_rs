//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 739/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk739<F: Float>(t7773: F, t1985: F, t7637: F, t7508: F, t56: F, t593: F, t151: F) -> (F, F, F, F, F) {
    let t7774 = F::cast_from(0.12862205435420921092e-2_f64) * t7773;
    let t7775 = t7637 * t1985;
    let t7776 = F::cast_from(0.95275595817932748827e-3_f64) * t7775;
    let t7777 = F::cast_from(1.0_f64) / t7508;
    let t7778 = t7777 * t56;
    let t7779 = t593 * t7778;
    let t7780 = t151 * t7779;
    (t7774, t7776, t7777, t7779, t7780)
}
