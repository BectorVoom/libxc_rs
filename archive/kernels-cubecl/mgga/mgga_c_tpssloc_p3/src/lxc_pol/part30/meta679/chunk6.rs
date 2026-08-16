//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2133/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2133<F: Float>(t225: F, t28051: F, t1386: F, t20044: F, t2016: F, t28187: F, t3758: F, t56640: F, t6993: F, t90525: F, t90534: F, t90542: F, t90547: F, t90550: F, t96905: F, t96910: F) -> F {
    let t96913 = t28051 * t225;
    let t96917 = -F::cast_from(0.16449340668482264365e-1_f64) * t96905 - t90525 + t90534 + t90542 - F::cast_from(0.49348022005446793095e-1_f64) * t96910 - t3758 * t28187 + t90547 - t96913 * t1386 - t56640 * t2016 - t90550 - t20044 * t6993;
    t96917
}
