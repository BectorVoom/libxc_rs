//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 563/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk563<F: Float>(t2450: F, t3402: F, t1137: F, t1324: F, t1140: F, t1328: F, t1350: F, t398: F, t429: F, t384: F, t513: F, t879: F) -> (F, F, F, F, F, F) {
    let t4463 = t2450 * t3402;
    let t4492 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t1137 * t1324;
    let t4494 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t1140 * t1328;
    let t4503 = t398 * t429 * t1350;
    let t4505 = F::cast_from(0.85748036236139473944e-3_f64) * t384 * t4503;
    let t4521 = t513 * t879;
    (t4463, t4492, t4494, t4503, t4505, t4521)
}
