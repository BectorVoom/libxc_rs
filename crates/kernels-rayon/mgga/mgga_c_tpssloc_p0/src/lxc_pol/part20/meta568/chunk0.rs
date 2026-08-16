//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2128/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2128(t3147: f64, t698: f64, t973: f64, t10981: f64, t2960: f64, t10984: f64, t1004: f64, t10956: f64, t10863: f64, t3053: f64, t10516: f64, t3113: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42613 = t973 * t698 * t3147;
    let t42619 = t2960 * t10981;
    let t42622 = t2960 * t10984;
    let t42648 = t1004 * t10956;
    let t42651 = t10863 * t3053;
    let t42653 = t3113 * t10516;
    (t42613, t42619, t42622, t42648, t42651, t42653)
}
