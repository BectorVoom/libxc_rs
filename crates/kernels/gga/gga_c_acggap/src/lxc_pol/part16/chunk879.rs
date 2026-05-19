//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 879/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk879<F: Float>(t30401: F, t30409: F, t30418: F, t322: F, t130: F, t3558: F, t145: F, t154: F, t19: F, t3157: F, t661: F, t1165: F, t3809: F, t604: F, t7493: F) -> (F, F, F, F) {
    let t30421 = t30401 * t30418 * t30409 * t322;
    let t30422 = F::cast_from(0.1886885537376249124e-2_f64) * t30421;
    let t30423 = t130 * t3558;
    let t30428 = t30423 * t154 * t3157 * t145 * t19 * t661;
    let t30429 = F::new(5.0) / F::new(576.0) * t30428;
    let t30444 = t7493 * t1165 * t604 * t3809;
    (t30422, t30423, t30429, t30444)
}
