//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 411/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk411<F: Float>(t188: F, t3371: F, t2898: F, t901: F, t1645: F, t888: F) -> (F, F, F) {
    let t3372 = t188 * t3371;
    let t3375 = t2898 * t901;
    let t3376 = F::new(0.14896037479937677779e-1) * t3375;
    let t3377 = t1645 * t888;
    (t3372, t3376, t3377)
}
