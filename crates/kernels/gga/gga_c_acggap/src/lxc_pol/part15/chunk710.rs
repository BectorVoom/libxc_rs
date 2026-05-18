//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 710/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk710<F: Float>(t606: F, t7630: F, t7508: F, t8: F, t56: F, t593: F, t151: F) -> (F, F, F, F) {
    let t7631 = t7630 * t606;
    let t7634 = F::new(1.0) / t8 / t7508;
    let t7635 = t7634 * t56;
    let t7636 = t593 * t7635;
    let t7637 = t151 * t7636;
    (t7631, t7634, t7636, t7637)
}
