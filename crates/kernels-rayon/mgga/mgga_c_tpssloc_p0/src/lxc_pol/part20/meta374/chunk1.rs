//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1724/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1724(t2563: f64, t4138: f64, t4134: f64, t9546: f64, t118: f64, t4119: f64, t794: f64, t2576: f64, t13005: f64, t13007: f64, t13010: f64, t13014: f64, t13017: f64, t787: f64, t9572: f64, t9574: f64, t9579: f64, t9583: f64) -> (f64, f64) {
    let t13020 = t2563 * t4138;
    let t13022 = t9546 * t4134;
    let t13025 = t118 * t794 * t4119;
    let t13027 = 0.16666666666666666666e-2_f64 * t2576 * t13025;
    let t13028 = -0.19999999999999999999e-1_f64 * t13005 * t13007 - t9572 - 0.12962962962962962962e-1_f64 * t13010 - t13014 - 0.11666666666666666666e-1_f64 * t9574 + t9579 - 0.16666666666666666666e-2_f64 * t787 * t13017 + 0.77777777777777777774e-2_f64 * t13020 - 0.52777777777777777776e-2_f64 * t13022 + t13027 - t9583;
    (t13025, t13028)
}
