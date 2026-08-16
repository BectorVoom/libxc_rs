//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 525/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk525<F: Float>(t10116: F, t2268: F, t3371: F, t448: F, t203: F, t3338: F) -> (F, F, F) {
    let t10118 = F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t10116;
    let t10119 = t3371 * t448;
    let t10122 = t203 * t3338;
    (t10118, t10119, t10122)
}
