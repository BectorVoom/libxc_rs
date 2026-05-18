//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1302/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1302<F: Float>(t18210: F, t29525: F, t7968: F, t1464: F, t1489: F, t7392: F, t98310: F, t1497: F, t28503: F, t60756: F, t28720: F, t6140: F) -> (F, F, F, F, F) {
    let t102294 = t18210 * t29525;
    let t102295 = t7968 * t102294;
    let t102299 = t1464 * t98310 * t7392 * t1489;
    let t102303 = t1464 * t28503 * t60756 * t1497;
    let t102305 = t28720 * t6140;
    (t102294, t102295, t102299, t102303, t102305)
}
