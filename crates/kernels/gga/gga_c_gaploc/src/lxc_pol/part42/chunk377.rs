//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 377/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk377<F: Float>(t169: F, t3614: F, t299: F, t706: F, t325: F, t3602: F, t738: F, t1890: F, t3601: F) -> (F, F, F, F, F, F) {
    let t3615 = t3614 * t169;
    let t3616 = t3615 * t299;
    let t3617 = t706 * t3616;
    let t3621 = t3602 * t325;
    let t3622 = t738 * t3621;
    let t3626 = t1890 * t3601;
    (t3615, t3616, t3617, t3621, t3622, t3626)
}
