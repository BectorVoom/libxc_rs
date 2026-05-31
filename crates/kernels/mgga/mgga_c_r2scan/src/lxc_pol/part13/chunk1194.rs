//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1194/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1194<F: Float>(t11855: F, t498: F, t3262: F, t3264: F, t11556: F, t37271: F, t10954: F, t11564: F, t3446: F, t11015: F, t11568: F, t3434: F) -> (F, F, F, F, F) {
    let t40324 = t498 * t11855;
    let t40327 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t3262 * t40324 * t3264;
    let t40329 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t37271 * t11556;
    let t40331 = t3446 * t10954 * t11564;
    let t40334 = t3434 * t11015 * t11568;
    (t40324, t40327, t40329, t40331, t40334)
}
