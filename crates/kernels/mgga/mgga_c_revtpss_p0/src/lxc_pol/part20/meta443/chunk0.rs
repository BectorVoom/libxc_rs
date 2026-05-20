//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1694/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1694<F: Float>(t2289: F, t2367: F, t10210: F, t625: F, t10214: F, t10255: F, t10207: F, t111: F, t2340: F, t2366: F, t39455: F, t36227: F) -> (F, F, F, F, F, F, F, F, F) {
    let t46148 = t2289 * t2367;
    let t46150 = t625 * t10210;
    let t46152 = t625 * t10214;
    let t46154 = t625 * t10255;
    let t46157 = F::new(1.0) / t10207 / t111;
    let t46158 = t2340 * t2340;
    let t46166 = t2366 * t2366;
    let t46173 = -F::new(12.0) * t39455;
    let t46196 = F::new(1.0) / t36227;
    (t46148, t46150, t46152, t46154, t46157, t46158, t46166, t46173, t46196)
}
