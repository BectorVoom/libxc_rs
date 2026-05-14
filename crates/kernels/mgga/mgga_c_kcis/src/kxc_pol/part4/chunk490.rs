//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 490/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk490<F: Float>(t684: F, t687: F, t686: F, t81: F, t60: F, t705: F, t78: F, t159: F, t9: F) -> (F, F, F, F, F, F, F, F) {
    let t2381 = t684 * t687;
    let t2385 = 1.0 / t686 / t81;
    let t2386 = t60 * t2385;
    let t2387 = t705 * t705;
    let t2388 = t78 * t78;
    let t2389 = 1.0 / t2388;
    let t2390 = t2387 * t2389;
    let t2394 = 1.0 / t9 / t159;
    (t2381, t2385, t2386, t2387, t2388, t2389, t2390, t2394)
}
