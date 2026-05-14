//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 421/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk421<F: Float>(t158: F, t2293: F, t2353: F, t501: F, t1381: F, t892: F, t60: F, t78: F) -> (F, F, F, F) {
    let t6540 = t158 * t2293;
    let t6553 = t2353 * t501;
    let t6556 = t892 * t1381;
    let t6574 = t60 * t78;
    (t6540, t6553, t6556, t6574)
}
