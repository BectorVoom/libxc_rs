//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1693/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1693<F: Float>(t5: F, t46123: F, t117: F, t10414: F, t116: F, t2319: F, t2327: F, t2371: F, t112: F, t46089: F, t10199: F, t666: F, t2289: F, t2341: F) -> (F, F, F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t46124 = piecewise3::<F>(t8, F::new(0.0), t46123);
    let t46125 = t46124 * t117;
    let t46126 = t10414 * t116;
    let t46129 = t2319 * t2327;
    let t46137 = t2371 * t2371;
    let t46143 = F::new(2618.0) / F::new(81.0) * t46089 * t112;
    let t46144 = t10199 * t666;
    let t46146 = t2289 * t2341;
    (t46125, t46126, t46129, t46137, t46143, t46144, t46146)
}
