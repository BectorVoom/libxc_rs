//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 803/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk803(t1647: f64, t3445: f64, t2221: f64, t3590: f64, t558: f64, t574: f64, t2142: f64, t3565: f64, t144: f64, t1053: f64, t9428: f64, t1882: f64, t3480: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12625 = t3445 * t1647;
    let t12626 = t2221 * t12625;
    let t12630 = t574 * t3590 * t558;
    let t12633 = t2142 * t3565;
    let t12634 = t144 * t12633;
    let t12637 = t9428 * t1053;
    let t12638 = t144 * t12637;
    let t12642 = 2.0_f64 / 9.0_f64 * t1882 * t3480;
    (t12626, t12630, t12633, t12634, t12637, t12638, t12642)
}
