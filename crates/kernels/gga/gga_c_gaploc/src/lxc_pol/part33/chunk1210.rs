//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1210/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1210<F: Float>(t34070: F, t34074: F, t34078: F, t34087: F, t34092: F, t34094: F, t34096: F, t34098: F, t34100: F, t34106: F, t34108: F, t34119: F, t34121: F, t34123: F, t34125: F, t34128: F) -> (F,) {
    let t38475 = -t34070 - t34074 - t34078 - t34087 + t34092 + t34094 + t34096 + t34098 + t34100 - t34106 + t34108 + t34119 + t34121 + t34123 + t34125 + t34128;
    (t38475,)
}
