//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1298/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1298<F: Float>(t1092: F, t14628: F, t26760: F, t3190: F, t27768: F, t92701: F, t1245: F, t27807: F, t291: F, t27812: F, t283: F, t5168: F) -> (F, F, F, F, F) {
    let t95884 = t1092 * t26760 * t14628 * t3190;
    let t95887 = t1092 * t92701 * t27768;
    let t95890 = t1245 * t291 * t27807;
    let t95892 = F::new(0.12378114784505208333e-4) * t27812 * t95890;
    let t95893 = t5168 * t283;
    (t95884, t95887, t95890, t95892, t95893)
}
