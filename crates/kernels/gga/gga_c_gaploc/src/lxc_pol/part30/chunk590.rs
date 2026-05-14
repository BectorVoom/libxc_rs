//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 590/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk590<F: Float>(t331: F, t3363: F, t3364: F, t3365: F, t3368: F, t3419: F, t3455: F, t3457: F, t3458: F, t3461: F, t3511: F, t748: F, t122: F, t1220: F, t481: F) -> (F, F) {
    let t3513 = t331 * t3455 - t3511 * t748 - t3363 + t3364 + t3365 - t3368 + t3419 - t3457 - t3458 + t3461;
    let t3808 = t481 * t1220 * t122;
    (t3513, t3808)
}
