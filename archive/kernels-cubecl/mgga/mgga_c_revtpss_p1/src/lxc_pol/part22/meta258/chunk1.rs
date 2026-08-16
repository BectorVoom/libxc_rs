//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1589/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1589<F: Float>(t3433: F, t6474: F, t3439: F, t5044: F, t6423: F, t6427: F, t6431: F, t1744: F, t1169: F) -> (F, F, F, F) {
    let t6476 = F::cast_from(0.16081979498692535067e2_f64) * t3433 * t6474;
    let t6481 = t3439 - F::cast_from(0.11415555555555555555e-1_f64) * t5044 - F::cast_from(0.11415555555555555555e-1_f64) * t6423 + F::cast_from(0.34246666666666666666e-1_f64) * t6427 + F::cast_from(0.17123333333333333333e-1_f64) * t6431;
    let t6486 = t1744 * t1744;
    let t6487 = t6486 * t1169;
    (t6476, t6481, t6486, t6487)
}
