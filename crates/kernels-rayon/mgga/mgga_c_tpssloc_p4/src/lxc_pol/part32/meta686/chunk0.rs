//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2128/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2128(t26135: f64, t7676: f64, t2314: f64, t28017: f64, t5113: f64, t1873: f64, t96356: f64, t28002: f64, t6534: f64, t12725: f64, t7467: f64, t75560: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t96667 = 4.0_f64 * t7676 * t26135;
    let t96669 = 2.0_f64 * t2314 * t28017;
    let t96671 = 2.0_f64 * t5113 * t28017;
    let t96673 = 4.0_f64 * t96356 * t1873;
    let t96675 = 4.0_f64 * t28002 * t6534;
    let t96677 = 4.0_f64 * t12725 * t7467;
    let t96679 = 2.0_f64 * t75560 * t1873;
    (t96667, t96669, t96671, t96673, t96675, t96677, t96679)
}
