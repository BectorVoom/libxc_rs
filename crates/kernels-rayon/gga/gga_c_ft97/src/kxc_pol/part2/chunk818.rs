//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 818/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk818(t1775: f64, t3500: f64, t12330: f64, t2102: f64, t12283: f64, t12288: f64, t9192: f64, t3515: f64, t1033: f64, t8282: f64, t1986: f64, t3518: f64, t9016: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12839 = 4.0_f64 / 27.0_f64 * t1775 * t3500;
    let t12840 = t2102 * t12330;
    let t12843 = t2102 * t12283;
    let t12846 = t9192 * t12288;
    let t12850 = 2.0_f64 / 9.0_f64 * t1775 * t3515;
    let t12852 = t8282 * t1033;
    let t12855 = t9016 * t3518 * t1986;
    (t12839, t12840, t12843, t12846, t12850, t12852, t12855)
}
