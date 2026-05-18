//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 902/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk902<F: Float>(t41698: F, t38051: F, t544: F, t9287: F, t13398: F, t7014: F, t11172: F, t2464: F, t2465: F, t2487: F, t11386: F, t2437: F) -> (F, F, F, F, F) {
    let t46176 = F::new(0.20449560508757733161e1) * t41698;
    let t46189 = t544 * t38051 * t9287;
    let t46190 = F::new(0.14896037479937677779e-1) * t46189;
    let t46191 = t7014 * t13398;
    let t46195 = t2487 * t2464 * t2465 * t11172;
    let t46212 = F::new(0.35750489951850426669e0) * t2437 * t11386;
    (t46176, t46190, t46191, t46195, t46212)
}
