//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 805/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk805(t12610: f64, t12614: f64, t12617: f64, t12620: f64, t12622: f64, t12626: f64, t12630: f64, t12634: f64, t12638: f64, t12642: f64, t12644: f64, t12647: f64, t12652: f64, t12656: f64, t12660: f64, t1901: f64, t446: f64) -> f64 {
    let t12663 = -2.0_f64 / 9.0_f64 * t1901 * t12610 + 2.0_f64 / 3.0_f64 * t446 * t12614 - 4.0_f64 / 81.0_f64 * t12617 + t12620 - 2.0_f64 / 9.0_f64 * t446 * t12622 - 2.0_f64 / 9.0_f64 * t1901 * t12626 - 2.0_f64 / 3.0_f64 * t446 * t12630 - 2.0_f64 / 3.0_f64 * t446 * t12634 - t446 * t12638 / 3.0_f64 - t12642 - t12644 + 4.0_f64 / 3.0_f64 * t446 * t12647 + 2.0_f64 / 3.0_f64 * t446 * t12652 + 4.0_f64 / 3.0_f64 * t446 * t12656 + 2.0_f64 / 3.0_f64 * t446 * t12660;
    t12663
}
