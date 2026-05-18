//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 824/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk824<F: Float>(t2798: F, t9588: F, t10295: F, t19933: F, t24215: F, t3366: F, t3207: F, t8042: F, t1016: F, t29096: F, t12960: F, t1537: F) -> (F, F, F, F, F, F) {
    let t41575 = t2798 * t9588;
    let t41579 = F::new(12.0) * t19933 * t10295;
    let t41581 = F::new(4.0) * t24215 * t3366;
    let t41585 = t8042 * t3207;
    let t41586 = t29096 * t1016;
    let t41594 = t1537 * t12960;
    (t41575, t41579, t41581, t41585, t41586, t41594)
}
