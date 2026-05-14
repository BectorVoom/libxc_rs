//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1033/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1033<F: Float>(t25374: F, t8392: F, t1882: F, t25227: F, t6280: F, t8232: F, t6289: F, t6347: F, t848: F, t25210: F, t25370: F, t25273: F, t10491: F, t1495: F, t2842: F, t6386: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t99016 = t8392 * t25374;
    let t99025 = t1882 * t25227;
    let t99030 = t8232 * t6280;
    let t99032 = t8232 * t6289;
    let t99034 = t848 * t6347;
    let t99076 = t1882 * t25210;
    let t99090 = t8392 * t25370;
    let t99092 = t8392 * t25273;
    let t99098 = t10491 * t1495;
    let t99102 = t2842 * t6386;
    (t99016, t99025, t99030, t99032, t99034, t99076, t99090, t99092, t99098, t99102)
}
