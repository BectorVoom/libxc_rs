//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 286/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk286(t1147: f64, t737: f64, t1131: f64, t743: f64, t192: f64, t462: f64, t736: f64, t92: f64, t734: f64, t91: f64, t1089: f64, t1134: f64, t751: f64) -> (f64, f64, f64, f64, f64) {
    let t1148 = t737 * t1147;
    let t1151 = t743 * t1131;
    let t1152 = t192 * t1151;
    let t1154 = -t736 - t462 * t1148 / 3.0_f64 - t92 * t1152;
    let t1156 = t91 * t734 * t1154;
    let t1160 = t1156 / 6.0_f64 - t751 - t1089 / 9.0_f64 - t1134 / 3.0_f64;
    (t1148, t1152, t1154, t1156, t1160)
}
