//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1101/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1101(t26564: f64, t7309: f64, t26574: f64, t1349: f64, t138549: f64, t139418: f64, t26540: f64, t26553: f64, t26771: f64, t28: f64, t32686: f64, t32738: f64, t32998: f64, t35007: f64, t5781: f64, t5843: f64, t6580: f64, t6587: f64, t6622: f64, t6723: f64) -> f64 {
    let t147198 = t7309 * t26564;
    let t147216 = t7309 * t26574;
    let t147224 = -t147198 / 18.0_f64 + t1349 * t28 * t5843 * t6723 / 3.0_f64 - t7309 * t26553 / 3.0_f64 + t138549 / 9.0_f64 + t6580 * t32998 - 2.0_f64 / 3.0_f64 * t6580 * t32738 - t35007 * t5781 / 3.0_f64 - t7309 * t26540 / 3.0_f64 + t7309 * t26771 / 6.0_f64 - t147216 / 18.0_f64 + t32686 * t6622 / 6.0_f64 - t1349 * t28 * t139418 * t6587 / 3.0_f64;
    t147224
}
