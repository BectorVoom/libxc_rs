//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 527/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk527<F: Float>(t1039: F, t3216: F, t105: F, t166: F, t1: F, t383: F, t980: F) -> (F, F, F, F) {
    let t3218 = F::new(0.60023625365297631762e-2) * t3216 * t1039;
    let t3220 = F::new(1.0) / t166 / t105;
    let t3221 = t3220 * t1;
    let t3228 = t980 * t383;
    (t3218, t3220, t3221, t3228)
}
