//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 936/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk936<F: Float>(t31340: F, t1121: F, t2020: F, t1133: F, t2450: F, t7560: F, t3401: F, t7559: F, t1170: F, t3378: F, t7336: F) -> (F, F, F, F, F, F, F) {
    let t31341 = F::cast_from(0.573046875e-1_f64) * t31340;
    let t31342 = t2020 * t1121;
    let t31344 = t2020 * t1133;
    let t31346 = t2450 * t7560;
    let t31349 = t7559 * t3401;
    let t31350 = t1170 * t31349;
    let t31362 = t3378 * t7336;
    (t31341, t31342, t31344, t31346, t31349, t31350, t31362)
}
