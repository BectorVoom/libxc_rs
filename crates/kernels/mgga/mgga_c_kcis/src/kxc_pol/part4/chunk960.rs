//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 960/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk960<F: Float>(t13467: F, t2970: F, t4904: F, t743: F, t4907: F, t2635: F, t4580: F) -> (F, F, F, F) {
    let t13468 = t2970 * t13467;
    let t13472 = 0.4705225e-4 * t743 * t4904;
    let t13473 = t743 * t4907;
    let t13475 = t4580 * t2635;
    (t13468, t13472, t13473, t13475)
}
