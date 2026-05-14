//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 600/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk600<F: Float>(t611: F, t8232: F, t2178: F, t597: F, t571: F, t1637: F, t599: F, t89: F, t143: F, t7954: F, t9065: F, t8796: F, t9071: F, t2101: F, t24: F, t7368: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9272 = t8232 * t611;
    let t9276 = t597 * t2178;
    let t9298 = t8232 * t571;
    let t9321 = t89 * t1637 * t599;
    let t9327 = t7954 * t143;
    let t9369 = 4.0 / 9.0 * t9065;
    let t9371 = 4.0 / 27.0 * t8796;
    let t9383 = 28.0 / 81.0 * t9071;
    let t9419 = t2101 * t597;
    let t9432 = t24 * t7368;
    (t9272, t9276, t9298, t9321, t9327, t9369, t9371, t9383, t9419, t9432)
}
