//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 991/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk991<F: Float>(t21125: F, t5968: F, t17594: F, t21130: F, t21134: F, t1392: F, t1979: F, t5441: F, t3751: F, t5427: F, t21106: F, t5976: F) -> (F, F, F, F, F, F) {
    let t22582 = t5968 * t21125;
    let t22585 = t17594 * t21130;
    let t22588 = t5968 * t21134;
    let t22591 = t1392 * t1979;
    let t22592 = t22591 * t5441;
    let t22595 = t3751 * t1979;
    let t22596 = t22595 * t5427;
    let t22601 = t5976 * t21106;
    (t22582, t22585, t22588, t22592, t22596, t22601)
}
