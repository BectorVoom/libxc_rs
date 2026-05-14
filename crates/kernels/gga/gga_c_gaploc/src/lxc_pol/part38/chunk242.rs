//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 242/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk242<F: Float>(t284: F, t712: F, t293: F, t711: F, t291: F, t279: F, t481: F, t729: F) -> (F, F, F, F, F, F) {
    let t1683 = t284 * t284;
    let t1685 = 1.0 / t1683 / t284;
    let t1687 = t1685 * M_PI * t712;
    let t1691 = 1.0 / t711 / t293;
    let t1692 = t291 * t1691;
    let t1841 = t481 * t729 * t279;
    (t1683, t1685, t1687, t1691, t1692, t1841)
}
