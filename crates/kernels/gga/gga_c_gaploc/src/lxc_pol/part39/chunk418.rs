//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 418/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk418<F: Float>(t2562: F, t3443: F, t943: F, t2958: F, t935: F) -> (F, F, F) {
    let t3444 = t2562 * t3443;
    let t3445 = t943 * t3444;
    let t3446 = F::new(0.32043859292259267849e-3) * t3445;
    let t3447 = t2958 * t935;
    (t3444, t3446, t3447)
}
