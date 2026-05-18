//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 577/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk577<F: Float>(t1184: F, t3409: F, t1190: F, t1165: F, t407: F, t991: F, t1163: F, t1171: F, t3370: F) -> (F, F, F, F, F) {
    let t3410 = t3409 * t1184;
    let t3412 = t3409 * t1190;
    let t3427 = t1165 * t991 * t407;
    let t3428 = t1163 * t3427;
    let t3430 = t3370 * t1171;
    (t3410, t3412, t3427, t3428, t3430)
}
