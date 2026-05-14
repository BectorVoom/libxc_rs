//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 599/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk599<F: Float>(t8805: F, t9068: F, t8799: F, t8802: F, t9059: F, t9071: F, t9062: F, t1882: F, t2198: F, t2101: F, t597: F, t2133: F, t604: F, t24: F, t7368: F, t603: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9366 = 2.0 / 3.0 * t8805;
    let t9370 = t9068 / 3.0;
    let t9372 = t8799 / 9.0;
    let t9373 = 2.0 / 27.0 * t8802;
    let t9380 = 2.0 / 9.0 * t9059;
    let t9383 = 28.0 / 81.0 * t9071;
    let t9390 = 2.0 / 9.0 * t9062;
    let t9405 = t1882 * t2198;
    let t9419 = t2101 * t597;
    let t9428 = t2133 * t604;
    let t9432 = t24 * t7368;
    let t9437 = t603 * t603;
    (t9366, t9370, t9372, t9373, t9380, t9383, t9390, t9405, t9419, t9428, t9432, t9437)
}
