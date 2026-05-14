//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 792/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk792<F: Float>(t2482: F, t3541: F, t9267: F, t11549: F, t9272: F, t11402: F, t2441: F, t13397: F, t21373: F, t6914: F, t11218: F, t123: F, t883: F, t2487: F, t2488: F, t11254: F, t2464: F, t2465: F) -> (F, F, F, F, F, F, F) {
    let t46387 = t9267 * t3541 * t2482;
    let t46390 = t9272 * t11549 * t2482;
    let t46396 = 0.35750489951850426669e0 * t2441 * t11402;
    let t46398 = t6914 * t21373 * t13397;
    let t46401 = t11218 * t123 * t883;
    let t46403 = t2487 * t2488 * t46401;
    let t46404 = 0.19171462976960374838e0 * t46403;
    let t46407 = t2487 * t2464 * t2465 * t11254;
    (t46387, t46390, t46396, t46398, t46401, t46404, t46407)
}
