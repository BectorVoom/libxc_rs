//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2367/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2367<F: Float>(t1858: F, t8110: F, t29865: F, t580: F, t2169: F, t6483: F, t29884: F, t576: F, t20186: F, t2170: F, t27908: F, t5381: F, t6471: F, t7426: F, t8111: F, t96289: F, t96291: F, t96300: F, t96303: F, t96308: F) -> F {
    let t105144 = t8110 * t1858;
    let t105146 = t29865 * t580;
    let t105147 = t2169 * t6483;
    let t105150 = t576 * t29884;
    let t105151 = F::cast_from(2.0_f64) * t1858 * t27908 + t20186 * t2170 + F::cast_from(2.0_f64) * t5381 * t8111 + t6471 * t7426 + F::cast_from(2.0_f64) * t105144 + t105146 + t105147 + t105150 + t96289 + t96291 + t96300 + t96303 + t96308;
    t105151
}
