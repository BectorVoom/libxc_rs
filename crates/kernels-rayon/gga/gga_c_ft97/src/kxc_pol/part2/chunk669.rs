//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 669/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk669(t9723: f64, t9727: f64, t9730: f64, t9520: f64, t9768: f64, t9765: f64, t251: f64, t631: f64, t675: f64, t7242: f64, t898: f64, t2371: f64, t665: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9861 = t9723 / 9.0_f64;
    let t9862 = 2.0_f64 / 27.0_f64 * t9727;
    let t9869 = 2.0_f64 / 3.0_f64 * t9730;
    let t9870 = t9520 / 3.0_f64;
    let t9872 = 2.0_f64 / 9.0_f64 * t9768;
    let t9876 = 2.0_f64 / 9.0_f64 * t9765;
    let t9890 = 1.0_f64 / t251 / t631 / t898 / t675 / t7242 / 4.0_f64;
    let t9895 = t665 * t2371;
    (t9861, t9862, t9869, t9870, t9872, t9876, t9890, t9895)
}
