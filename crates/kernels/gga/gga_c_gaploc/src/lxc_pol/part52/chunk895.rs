//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 895/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk895<F: Float>(t46044: F, t2478: F, t3536: F, t6576: F, t37977: F, t44255: F, t549: F, t20367: F, t44387: F, t4820: F, t2375: F, t37575: F) -> (F, F, F, F, F) {
    let t46045 = F::new(0.9585731488480187419e0) * t46044;
    let t46047 = t6576 * t3536 * t2478;
    let t46052 = F::new(0.47667319935800568892e0) * t37977 * t549 * t44255;
    let t46055 = F::new(0.23833659967900284446e0) * t20367 * t4820 * t44387;
    let t46057 = F::new(0.11916829983950142223e0) * t37575 * t2375;
    (t46045, t46047, t46052, t46055, t46057)
}
