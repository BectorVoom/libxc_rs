//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 766/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk766(t292: f64, t10292: f64, t10297: f64, t10359: f64, t10364: f64, t10365: f64, t10369: f64, t10384: f64, t2688: f64, t2691: f64, t2692: f64, t2720: f64, t2726: f64, t2735: f64, t285: f64, t4061: f64, t4113: f64, t800: f64, t817: f64, t821: f64) -> f64 {
    let t293 = 0.1e-59_f64 < t292;
    let t10388 = piecewise3(t293, 12.0_f64 * t10292 * t2691 * t2726 - 6.0_f64 * t10364 * t10365 * t285 + 6.0_f64 * t10369 * t2735 * t4113 - t10384 * t285 * t817 - 6.0_f64 * t2691 * t2692 * t2735 - 6.0_f64 * t10297 * t2691 + 2.0_f64 * t10359 * t800 - 6.0_f64 * t2688 * t821 + 6.0_f64 * t2720 * t4061, 0.0_f64);
    t10388
}
