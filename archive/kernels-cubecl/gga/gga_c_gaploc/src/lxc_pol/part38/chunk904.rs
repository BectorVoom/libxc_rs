//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 904/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk904<F: Float>(t43522: F, t43526: F, t44707: F, t723: F) -> (F, F, F) {
    let t45366 = F::cast_from(0.59584149919750711116e-1_f64) * t43522;
    let t45367 = F::cast_from(0.59584149919750711116e-1_f64) * t43526;
    let t45369 = t44707 * t723;
    (t45366, t45367, t45369)
}
