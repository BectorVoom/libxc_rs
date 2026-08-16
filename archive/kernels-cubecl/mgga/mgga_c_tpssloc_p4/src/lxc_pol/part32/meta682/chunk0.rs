//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2122/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2122<F: Float>(t27930: F, t576: F, t112: F, t27907: F, t111: F, t8110: F, t28821: F, t6997: F, t1441: F, t4072: F, t1874: F, t28002: F, t6525: F) -> (F, F, F, F, F, F, F) {
    let t96308 = F::cast_from(2.0_f64) * t576 * t27930;
    let t96311 = t27907 * t112;
    let t96334 = t8110 * t111;
    let t96355 = t28821 * t6997;
    let t96356 = t1441 * t4072;
    let t96358 = F::cast_from(4.0_f64) * t96356 * t1874;
    let t96360 = F::cast_from(4.0_f64) * t28002 * t6525;
    (t96308, t96311, t96334, t96355, t96356, t96358, t96360)
}
