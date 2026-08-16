//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1065/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1065<F: Float>(t26592: F, t26631: F, t165: F, t2165: F, t2789: F, t26521: F, t26523: F, t26525: F, t26528: F, t26531: F, t26534: F, t26536: F, t26538: F, t26540: F, t26542: F, t26545: F, t26548: F, t26551: F, t26554: F) -> (F, F, F, F) {
    let t26632 = t26592 + t26631;
    let t26633 = t26632 * t165;
    let t26634 = t2165 * t2789;
    let t26651 = t26521 / F::cast_from(8.0_f64) - t26523 / F::cast_from(4.0_f64) - t26525 / F::cast_from(2.0_f64) + t26528 / F::cast_from(4.0_f64) + t26531 / F::cast_from(2.0_f64) - t26534 / F::cast_from(8.0_f64) + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t26536 - t26538 / F::cast_from(32.0_f64) + t26540 / F::cast_from(16.0_f64) + t26542 / F::cast_from(4.0_f64) - t26545 / F::cast_from(16.0_f64) - t26548 / F::cast_from(4.0_f64) + t26551 / F::cast_from(32.0_f64) - F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t26554;
    (t26632, t26633, t26634, t26651)
}
