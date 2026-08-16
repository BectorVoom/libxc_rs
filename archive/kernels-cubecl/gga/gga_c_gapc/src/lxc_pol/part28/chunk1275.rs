//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1275/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1275<F: Float>(t3691: F, t9160: F, t11362: F, t28169: F, t35112: F, t5218: F, t1044: F, t515: F, t169: F, t19: F, t3665: F, t116: F, t1882: F, t9092: F) -> (F, F, F, F, F, F) {
    let t35188 = t3691 * t9160;
    let t35190 = t11362 * t28169;
    let t35192 = t35112 * t5218;
    let t35194 = t515 * t1044;
    let t35197 = t169 * t35194 * t19 * t3665;
    let t35200 = t116 * t1882 * t9092;
    (t35188, t35190, t35192, t35194, t35197, t35200)
}
