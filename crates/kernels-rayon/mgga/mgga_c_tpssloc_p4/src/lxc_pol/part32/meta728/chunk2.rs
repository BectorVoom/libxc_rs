//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2367/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2367(t1858: f64, t8110: f64, t29865: f64, t580: f64, t2169: f64, t6483: f64, t29884: f64, t576: f64, t20186: f64, t2170: f64, t27908: f64, t5381: f64, t6471: f64, t7426: f64, t8111: f64, t96289: f64, t96291: f64, t96300: f64, t96303: f64, t96308: f64) -> f64 {
    let t105144 = t8110 * t1858;
    let t105146 = t29865 * t580;
    let t105147 = t2169 * t6483;
    let t105150 = t576 * t29884;
    let t105151 = 2.0_f64 * t1858 * t27908 + t20186 * t2170 + 2.0_f64 * t5381 * t8111 + t6471 * t7426 + 2.0_f64 * t105144 + t105146 + t105147 + t105150 + t96289 + t96291 + t96300 + t96303 + t96308;
    t105151
}
