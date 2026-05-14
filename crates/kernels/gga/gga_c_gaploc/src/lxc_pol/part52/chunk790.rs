//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 790/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk790<F: Float>(t46353: F, t2413: F, t37667: F, t11434: F, t2389: F, t44386: F, t475: F) -> (F, F, F, F) {
    let t46354 = 0.14896037479937677779e-1 * t46353;
    let t46356 = 0.25025342966295298669e1 * t37667 * t2413;
    let t46360 = t11434 * t2389;
    let t46361 = 0.29792074959875355558e-1 * t46360;
    let t46362 = t44386 * t475;
    (t46354, t46356, t46361, t46362)
}
