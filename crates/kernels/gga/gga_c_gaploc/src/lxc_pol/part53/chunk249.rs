//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 249/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk249<F: Float>(t284: F, t712: F, t293: F, t711: F, t291: F, t279: F, t481: F, t729: F) -> (F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t1683 = t284 * t284;
    let t1685 = F::cast_from(1.0_f64) / t1683 / t284;
    let t1687 = t1685 * pi * t712;
    let t1691 = F::cast_from(1.0_f64) / t711 / t293;
    let t1692 = t291 * t1691;
    let t1841 = t481 * t729 * t279;
    (t1683, t1685, t1687, t1691, t1692, t1841)
}
