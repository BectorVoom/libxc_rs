//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2118/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2118(t1873: f64, t96356: f64, t28002: f64, t6534: f64, t12725: f64, t7467: f64, t75560: f64, t19451: f64, t96654: f64, t96655: f64, t96659: f64, t96661: f64, t96663: f64, t96665: f64, t96667: f64, t96669: f64, t96671: f64) -> f64 {
    let t96673 = 4.0_f64 * t96356 * t1873;
    let t96675 = 4.0_f64 * t28002 * t6534;
    let t96677 = 4.0_f64 * t12725 * t7467;
    let t96679 = 2.0_f64 * t75560 * t1873;
    let t96681 = 2.0_f64 * t19451 * t6534;
    let t96682 = t96654 + 2.0_f64 * t96655 + t96659 + t96661 + t96663 + t96665 + t96667 + t96669 + t96671 + t96673 + t96675 + t96677 + t96679 + t96681;
    t96682
}
