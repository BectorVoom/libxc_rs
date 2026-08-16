//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 368/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk368<F: Float>(t227: F, t2063: F, t565: F, t806: F, t695: F, zeta_threshold: F) -> (F, F, F) {
    let t228 = t227 <= zeta_threshold;
    let t2359 = piecewise3::<F>(t228, F::cast_from(0.0_f64), t2063);
    let t2360 = t565 * t2359;
    let t2361 = t2360 * t806;
    let t2364 = t695 * t2063;
    (t2360, t2361, t2364)
}
