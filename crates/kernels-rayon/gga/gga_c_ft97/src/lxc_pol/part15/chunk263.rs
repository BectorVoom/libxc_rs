//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 263/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk263(t1091: f64, t265: f64, t724: f64, t1131: f64, t729: f64, t738: f64, t992: f64, t737: f64, t743: f64, t192: f64, t462: f64, t736: f64, t92: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1140 = t724 * t265 * t1091;
    let t1144 = t729 * t265 * t1131;
    let t1147 = t738 * t992;
    let t1148 = t737 * t1147;
    let t1151 = t743 * t1131;
    let t1152 = t192 * t1151;
    let t1154 = -t736 - t462 * t1148 / 3.0_f64 - t92 * t1152;
    (t1140, t1144, t1147, t1148, t1152, t1154)
}
