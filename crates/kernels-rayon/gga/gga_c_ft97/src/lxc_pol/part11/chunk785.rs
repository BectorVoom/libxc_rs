//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 785/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk785(t10588: f64, t10621: f64, t845: f64, t91: f64, t305: f64, t631: f64, t7242: f64, t798: f64, t898: f64, t2756: f64, t856: f64, t10246: f64) -> (f64, f64, f64, f64, f64) {
    let t10622 = t10588 + t10621;
    let t10624 = t91 * t845 * t10622;
    let t10631 = 1.0_f64 / t305 / t631 / t898 / t798 / t7242 / 4.0_f64;
    let t10632 = t2756 * t856;
    let t10634 = t91 * t10631 * t10632;
    let t10636 = 2.0_f64 / 9.0_f64 * t10246;
    (t10622, t10624, t10631, t10634, t10636)
}
