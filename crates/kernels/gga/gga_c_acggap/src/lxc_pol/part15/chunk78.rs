//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 78/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk78<F: Float>(t132: F, t203: F, t202: F, t195: F, t4: F, t11: F, t1: F, t21: F, t22: F, t5: F, t7: F) -> (F, F, F, F, F) {
    let t204 = t203 * t132;
    let t205 = t202 * t204;
    let t207 = t4 * t195;
    let t209 = F::sqrt(t11);
    let t210 = t209 * t1;
    let t211 = t210 * t204;
    let t216 = t21 * t5 / t22 / t7;
    (t205, t207, t210, t211, t216)
}
