//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 645/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk645<F: Float>(t2109: F, t832: F, t121: F, t2084: F, t1: F, t313: F, t191: F, t835: F) -> (F, F, F, F) {
    let t6024 = t2109 * t832;
    let t6058 = t121 * t2084;
    let t6059 = t6058 * t1;
    let t6060 = t313 * t6059;
    let t6066 = t191 * t835;
    (t6024, t6058, t6060, t6066)
}
