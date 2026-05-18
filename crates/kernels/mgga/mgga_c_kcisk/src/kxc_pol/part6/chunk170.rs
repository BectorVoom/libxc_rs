//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 170/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk170<F: Float>(t571: F, t574: F) -> (F, F, F, F) {
    let t677 = F::new(0.107924e1) + F::new(0.3964e-1) * t574 + F::new(0.123825e-1) * t571;
    let t680 = F::new(1.0) + t574 * t677 / F::new(2.0);
    let t681 = t680 * t680;
    let t682 = F::new(1.0) / t681;
    (t677, t680, t681, t682)
}
