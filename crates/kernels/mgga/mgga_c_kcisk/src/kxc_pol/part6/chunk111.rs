//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 111/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk111<F: Float>(t303: F, t306: F) -> (F, F, F, F) {
    let t420 = 0.107924e1 + 0.3964e-1 * t306 + 0.123825e-1 * t303;
    let t423 = 1.0 + t306 * t420 / 2.0;
    let t424 = t423 * t423;
    let t425 = 1.0 / t424;
    (t420, t423, t424, t425)
}
