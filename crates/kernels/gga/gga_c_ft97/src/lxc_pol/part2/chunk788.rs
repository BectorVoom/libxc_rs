//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 788/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk788<F: Float>(t8796: F, t1984: F, t3408: F, t558: F, t28: F, t89: F, t3343: F, t376: F, t11402: F, t3330: F, t7773: F, t998: F) -> (F, F, F, F, F, F) {
    let t12346 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t8796;
    let t12350 = t1984 * t3408;
    let t12351 = t12350 * t558;
    let t12353 = t89 * t28 * t12351;
    let t12356 = t89 * t376 * t3343;
    let t12357 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t12356;
    let t12359 = t89 * t11402 * t3330;
    let t12362 = t89 * t7773 * t998;
    (t12346, t12353, t12356, t12357, t12359, t12362)
}
