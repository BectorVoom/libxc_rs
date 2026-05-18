//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 486/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk486<F: Float>(t287: F, t3530: F, t2917: F, t1207: F, t1211: F, t1210: F, t401: F, t396: F, t2966: F, t404: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3531 = t287 * t3530;
    let t3537 = F::new(0.22831111111111111111e-1) * t2917;
    let t3545 = t1207 * t1211;
    let t3548 = t1210 * t401;
    let t3549 = F::new(1.0) / t3548;
    let t3550 = t396 * t3549;
    let t3557 = F::new(0.68863333333333333333e0) * t2917;
    let t3564 = F::new(0.17365833333333333333e0) * t2966;
    let t3573 = t1210 * t1210;
    let t3574 = F::new(1.0) / t3573;
    let t3575 = t396 * t3574;
    let t3576 = t404 * t404;
    let t3577 = F::new(1.0) / t3576;
    (t3531, t3537, t3545, t3549, t3550, t3557, t3564, t3573, t3574, t3575, t3576, t3577)
}
