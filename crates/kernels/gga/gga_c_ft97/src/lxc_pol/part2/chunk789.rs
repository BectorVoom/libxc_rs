//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 789/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk789<F: Float>(t3409: F, t375: F, t89: F, t3379: F, t549: F, t554: F, t2071: F, t3355: F, t135: F, t3347: F, t538: F, t1995: F, t3380: F) -> (F, F, F, F, F, F, F) {
    let t12365 = t89 * t375 * t3409;
    let t12366 = t12365 / F::cast_from(9.0_f64);
    let t12367 = t549 * t3379;
    let t12368 = t12367 * t554;
    let t12371 = t3355 * t2071;
    let t12374 = t3347 * t135;
    let t12381 = t538 * t554;
    let t12385 = t1995 * t3380;
    (t12365, t12366, t12368, t12371, t12374, t12381, t12385)
}
