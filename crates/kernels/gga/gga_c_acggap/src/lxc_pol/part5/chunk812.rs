//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 812/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk812<F: Float>(t1036: F, t1077: F, t368: F, t398: F, t864: F, t3764: F, t377: F, t409: F, t3372: F, t3445: F, t329: F, t3615: F, t124: F, t19: F, t7335: F, t1162: F, t12309: F) -> (F, F, F, F, F, F, F) {
    let t12529 = t1036 * t398 * t368 * t864 * t1077;
    let t12531 = t377 * t3764;
    let t12532 = t12531 * t409;
    let t12536 = t3372 * t3445;
    let t12572 = t329 * t3615;
    let t12576 = t124 * t7335 * t19;
    let t12586 = t12309 * t1162;
    (t12529, t12531, t12532, t12536, t12572, t12576, t12586)
}
