//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 314/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk314<F: Float>(t3455: F, t574: F, t605: F, t1026: F, t1882: F, t1060: F, t379: F, t569: F, t616: F, t925: F, t167: F, t3052: F) -> (F, F, F, F, F) {
    let t3457 = t574 * t605 * t3455;
    let t3460 = t1882 * t1026;
    let t3463 = t569 * t1060 * t379;
    let t3467 = t569 * t616 * t925;
    let t3471 = t569 * t167 * t3052;
    (t3457, t3460, t3463, t3467, t3471)
}
