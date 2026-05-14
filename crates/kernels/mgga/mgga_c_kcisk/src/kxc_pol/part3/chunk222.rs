//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 222/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk222<F: Float>(t167: F, t960: F, t965: F, t967: F, t970: F) -> (F,) {
    let t972 = 0.59778596625315888114e-2 * t167 - 0.17565e-2 * t960 + 0.39625e-3 * t965 - 0.1294884726949076719e-4 * t967 + 0.1260328125e-5 * t970;
    (t972,)
}
