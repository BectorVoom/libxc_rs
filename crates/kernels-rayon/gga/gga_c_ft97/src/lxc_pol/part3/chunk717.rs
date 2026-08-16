//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 717/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk717(t13730: f64, t3705: f64, t89: f64, t1132: f64, t1636: f64, t3718: f64, t681: f64, t375: f64, t3822: f64, t1882: f64, t3714: f64, t3692: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13732 = t89 * t13730 * t3705;
    let t13739 = t89 * t1636 * t1132;
    let t13740 = 4.0_f64 / 27.0_f64 * t13739;
    let t13746 = t89 * t681 * t3718;
    let t13747 = 4.0_f64 / 9.0_f64 * t13746;
    let t13753 = t89 * t375 * t3822;
    let t13754 = 2.0_f64 / 9.0_f64 * t13753;
    let t13780 = t1882 * t3714;
    let t13781 = 2.0_f64 / 27.0_f64 * t13780;
    let t13794 = t1882 * t3692;
    (t13732, t13739, t13740, t13746, t13747, t13753, t13754, t13780, t13781, t13794)
}
