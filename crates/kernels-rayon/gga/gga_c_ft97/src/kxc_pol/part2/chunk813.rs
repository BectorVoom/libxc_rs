//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 813/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk813(t12711: f64, t12716: f64, t12720: f64, t12726: f64, t12730: f64, t12734: f64, t12739: f64, t12743: f64, t12748: f64, t12752: f64, t12756: f64, t12759: f64, t12763: f64, t12767: f64, t12771: f64, t1901: f64, t446: f64) -> f64 {
    let t12774 = -4.0_f64 / 9.0_f64 * t1901 * t12711 + 4.0_f64 / 27.0_f64 * t1901 * t12716 - 2.0_f64 / 27.0_f64 * t1901 * t12720 - 10.0_f64 / 81.0_f64 * t1901 * t12726 + t1901 * t12730 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t1901 * t12734 + t1901 * t12739 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t12743 + 4.0_f64 / 9.0_f64 * t1901 * t12748 + 4.0_f64 / 27.0_f64 * t12752 + t446 * t12756 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t12759 - t446 * t12763 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t446 * t12767 - t446 * t12771 / 9.0_f64;
    t12774
}
