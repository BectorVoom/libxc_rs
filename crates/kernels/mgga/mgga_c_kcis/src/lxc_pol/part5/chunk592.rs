//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 592/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk592<F: Float>(t1210: F, t396: F, t404: F, t956: F, t962: F, t265: F, t3005: F, t3031: F, t187: F, t426: F, t1236: F, t1239: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3573 = t1210 * t1210;
    let t3574 = F::new(1.0) / t3573;
    let t3575 = t396 * t3574;
    let t3576 = t404 * t404;
    let t3577 = F::new(1.0) / t3576;
    let t3582 = t956 * t962;
    let t3585 = t265 * t3005;
    let t3592 = t265 * t3031;
    let t3600 = t187 * t956;
    let t3621 = t426 * t426;
    let t3622 = F::new(1.0) / t3621;
    let t3638 = t1236 * t1239;
    (t3573, t3574, t3575, t3576, t3577, t3582, t3585, t3592, t3600, t3621, t3622, t3638)
}
