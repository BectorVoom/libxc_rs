//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 910/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk910<F: Float>(t2912: F, t407: F, t1019: F, t2910: F, t2861: F, t3153: F, t475: F, t126: F, t3096: F, t215: F, t442: F, t68: F) -> (F, F, F, F, F, F) {
    let t9495 = F::cast_from(1.0_f64) / t2912 / t407;
    let t9504 = t1019 * t2910;
    let t9507 = t1019 * t2861;
    let t9519 = F::cast_from(1.0_f64) / t3153 / t475;
    let t9523 = t126 * t3096;
    let t9533 = t215 * t68 * t442;
    (t9495, t9504, t9507, t9519, t9523, t9533)
}
