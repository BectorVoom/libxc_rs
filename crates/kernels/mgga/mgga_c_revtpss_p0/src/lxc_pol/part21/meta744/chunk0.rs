//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2618/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2618<F: Float>(t13581: F, t177: F, t762: F, t46971: F, t1317: F, t13632: F, t3857: F, t5569: F, t512: F, t749: F, t46973: F, t3863: F, t5567: F) -> (F, F, F, F, F, F, F) {
    let t48222 = t13581 * t177 * t762;
    let t48223 = F::cast_from(0.17544670867903938621e1_f64) * t48222;
    let t48224 = F::new(480.0) * t46971;
    let t48225 = t1317 * t13632;
    let t48226 = F::new(12.0) * t48225;
    let t48227 = t3857 * t5569;
    let t48228 = F::new(60.0) * t48227;
    let t48230 = t512 * t13581 * t749;
    let t48231 = F::new(3.0) * t48230;
    let t48232 = F::new(36.0) * t46973;
    let t48234 = F::new(96.0) * t3863 * t5567;
    (t48223, t48224, t48226, t48228, t48231, t48232, t48234)
}
