//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 274/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk274<F: Float>(t167: F, t569: F, t925: F, t1017: F, t574: F, t582: F, t958: F, t586: F, t24: F, t462: F, t581: F, t92: F) -> (F, F, F, F, F) {
    let t1026 = t569 * t167 * t925;
    let t1030 = t574 * t167 * t1017;
    let t1033 = t582 * t958;
    let t1036 = t586 * t1017;
    let t1037 = t24 * t1036;
    let t1039 = -t581 - t462 * t1033 / F::new(3.0) - t92 * t1037;
    (t1026, t1030, t1033, t1037, t1039)
}
