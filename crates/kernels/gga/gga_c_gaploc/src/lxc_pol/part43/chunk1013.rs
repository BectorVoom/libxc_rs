//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1013/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1013<F: Float>(t12078: F, t1415: F, t7030: F, t47953: F, t6716: F, t6717: F, t13800: F, t4614: F, t574: F, t1445: F, t38413: F, t874: F) -> (F, F, F, F) {
    let t48208 = t1415 * t12078 * t7030;
    let t48211 = t6716 * t6717 * t47953;
    let t48217 = t574 * t4614 * t13800;
    let t48221 = t574 * t1445 * t38413 * t874;
    (t48208, t48211, t48217, t48221)
}
