//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 664/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk664(t113: f64, t2346: f64, t170: f64, t7512: f64, t195: f64, t25: f64, t209: f64, t2247: f64, t228: f64, t231: f64, t626: f64, t705: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9577 = 1.0_f64 / t2346 / t113;
    let t9606 = 4.0_f64 * t170 * t7512;
    let t9608 = 1.0_f64 / t195 / t9606;
    let t9609 = t25 * t9608;
    let t9634 = t209 * t2247;
    let t9636 = t228 * t9634 * t231;
    let t9637 = 0.70937342644032921812e-2_f64 * t9636;
    let t9638 = t626 * t705;
    (t9577, t9606, t9609, t9634, t9636, t9637, t9638)
}
