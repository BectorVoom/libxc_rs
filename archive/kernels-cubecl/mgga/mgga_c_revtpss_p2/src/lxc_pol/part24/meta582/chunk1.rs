//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1813/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1813<F: Float>(t543: F, t91921: F, t73321: F, t48152: F, t73329: F, t73331: F, t73341: F, t39419: F, t39422: F, t46292: F, t46297: F, t46303: F, t46963: F, t46970: F) -> (F, F, F, F, F, F, F) {
    let t91942 = t91921 * t543;
    let t91952 = F::cast_from(120.0_f64) * t73321;
    let t91953 = F::cast_from(48.0_f64) * t48152;
    let t91954 = F::cast_from(72.0_f64) * t73329;
    let t91955 = F::cast_from(192.0_f64) * t73331;
    let t91956 = F::cast_from(0.65061487801810439052e-1_f64) * t73341;
    let t91957 = t46292 - t46297 - t39419 - t39422 + t46303 + t91952 - t91953 + t91954 + t91955 - t46963 + t46970 + t91956;
    (t91942, t91952, t91953, t91954, t91955, t91956, t91957)
}
