//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 657/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk657<F: Float>(t1637: F, t599: F, t89: F, t143: F, t7954: F, t1882: F, t2144: F, t2170: F, t8805: F, t9068: F, t8799: F, t8802: F) -> (F, F, F, F, F, F, F, F) {
    let t9321 = t89 * t1637 * t599;
    let t9327 = t7954 * t143;
    let t9340 = t1882 * t2144;
    let t9342 = t1882 * t2170;
    let t9366 = F::new(2.0) / F::new(3.0) * t8805;
    let t9370 = t9068 / F::new(3.0);
    let t9372 = t8799 / F::new(9.0);
    let t9373 = F::new(2.0) / F::new(27.0) * t8802;
    (t9321, t9327, t9340, t9342, t9366, t9370, t9372, t9373)
}
