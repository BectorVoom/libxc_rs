//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 786/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk786<F: Float>(t2798: F, t9588: F, t10624: F, t2355: F, t10295: F, t19933: F, t24215: F, t3366: F, t12859: F, t4342: F, t3207: F, t8042: F, t1016: F, t29096: F, t10405: F, t2482: F, t9267: F) -> (F, F, F, F, F, F, F, F) {
    let t41575 = t2798 * t9588;
    let t41576 = t2355 * t10624;
    let t41579 = 12.0 * t19933 * t10295;
    let t41581 = 4.0 * t24215 * t3366;
    let t41583 = t4342 * t12859;
    let t41585 = t8042 * t3207;
    let t41586 = t29096 * t1016;
    let t41588 = t9267 * t10405 * t2482;
    (t41575, t41576, t41579, t41581, t41583, t41585, t41586, t41588)
}
