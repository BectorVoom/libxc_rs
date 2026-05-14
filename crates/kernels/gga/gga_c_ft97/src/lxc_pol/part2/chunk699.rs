//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 699/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk699<F: Float>(t12367: F, t554: F, t2071: F, t3355: F, t135: F, t3347: F, t538: F, t1995: F, t3380: F, t3405: F, t542: F, t1013: F, t8907: F, t1008: F, t2057: F, t2059: F) -> (F, F, F, F, F, F, F, F) {
    let t12368 = t12367 * t554;
    let t12371 = t3355 * t2071;
    let t12374 = t3347 * t135;
    let t12381 = t538 * t554;
    let t12385 = t1995 * t3380;
    let t12392 = t542 * t3405;
    let t12397 = t8907 * t1013;
    let t12401 = t2057 * t1008;
    let t12402 = t12401 * t2059;
    (t12368, t12371, t12374, t12381, t12385, t12392, t12397, t12402)
}
