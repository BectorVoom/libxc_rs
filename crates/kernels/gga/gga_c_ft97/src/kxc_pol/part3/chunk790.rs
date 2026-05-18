//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 790/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk790<F: Float>(t432: F, t4495: F, t110: F, t1871: F, t492: F, t452: F, t488: F, t3266: F, t986: F, t3214: F, t3238: F, t10969: F, t3219: F) -> (F, F, F, F, F) {
    let t16261 = t4495 * t432;
    let t16263 = t1871 * t110 * t16261;
    let t16266 = t4495 * t492;
    let t16268 = t452 * t488 * t16266;
    let t16272 = t1871 * t986 * t3266;
    let t16276 = t452 * t3238 * t3214;
    let t16279 = t10969 * t3219;
    (t16263, t16268, t16272, t16276, t16279)
}
