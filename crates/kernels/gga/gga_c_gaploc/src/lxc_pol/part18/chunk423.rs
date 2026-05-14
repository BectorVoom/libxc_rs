//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 423/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk423<F: Float>(t2095: F, t313: F, t191: F, t325: F, t107: F, t121: F, t830: F) -> (F, F, F, F) {
    let t2096 = t313 * t2095;
    let t2097 = t191 * t325;
    let t2098 = t107 * t2097;
    let t2101 = t121 * t830;
    (t2096, t2097, t2098, t2101)
}
