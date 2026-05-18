//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 661/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk661<F: Float>(t2178: F, t597: F, t571: F, t8232: F, t1637: F, t599: F, t89: F, t143: F, t7954: F, t9065: F, t8796: F, t9071: F) -> (F, F, F, F, F, F, F) {
    let t9276 = t597 * t2178;
    let t9298 = t8232 * t571;
    let t9321 = t89 * t1637 * t599;
    let t9327 = t7954 * t143;
    let t9369 = F::new(4.0) / F::new(9.0) * t9065;
    let t9371 = F::new(4.0) / F::new(27.0) * t8796;
    let t9383 = F::new(28.0) / F::new(81.0) * t9071;
    (t9276, t9298, t9321, t9327, t9369, t9371, t9383)
}
