//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1046/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1046<F: Float>(t35112: F, t5218: F, t1044: F, t515: F, t169: F, t19: F, t3665: F, t116: F, t1882: F, t9092: F, t11303: F, t21842: F, t11500: F, t1717: F, t144: F, t21072: F, t21076: F, t26416: F, t5542: F) -> (F, F, F, F, F, F, F) {
    let t35192 = t35112 * t5218;
    let t35194 = t515 * t1044;
    let t35197 = t169 * t35194 * t19 * t3665;
    let t35200 = t116 * t1882 * t9092;
    let t35203 = t11303 * t21842;
    let t35205 = t11500 * t1717;
    let t35210 = t21072 * t5542 * t26416 * t144 * t21076;
    (t35192, t35194, t35197, t35200, t35203, t35205, t35210)
}
