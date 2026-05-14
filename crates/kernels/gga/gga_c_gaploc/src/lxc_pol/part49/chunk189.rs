//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 189/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk189<F: Float>(t738: F, t948: F, t270: F, t938: F, t946: F, t314: F, t935: F) -> (F, F, F) {
    let t949 = t738 * t948;
    let t952 = 0.76905262301422242837e-2 * t270 * t938 + 0.32043859292259267849e-3 * t946 - 0.76905262301422242837e-2 * t270 * t949;
    let t954 = t314 * t935;
    (t949, t952, t954)
}
