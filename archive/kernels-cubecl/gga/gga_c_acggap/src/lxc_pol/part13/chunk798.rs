//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 798/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk798<F: Float>(t1554: F, t7822: F, t1558: F, t1421: F, t599: F, t1181: F, t7493: F, t1427: F, t8463: F, t1165: F, t1432: F, t7351: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8644 = t7822 * t1554;
    let t8646 = t7822 * t1558;
    let t8648 = t599 * t1421;
    let t8649 = t1181 * t8648;
    let t8650 = t7493 * t8649;
    let t8652 = t599 * t1427;
    let t8653 = t1181 * t8652;
    let t8654 = t8463 * t8653;
    let t8657 = t1165 * t7351 * t1432;
    (t8644, t8646, t8648, t8649, t8650, t8652, t8653, t8654, t8657)
}
