//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 885/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk885(t13739: f64, t2459: f64, t3717: f64, t193: f64, t89: f64, t3718: f64, t681: f64, t13672: f64, t676: f64, t27: f64, t375: f64, t3822: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13740 = 4.0_f64 / 27.0_f64 * t13739;
    let t13741 = t3717 * t2459;
    let t13743 = t89 * t193 * t13741;
    let t13746 = t89 * t681 * t3718;
    let t13747 = 4.0_f64 / 9.0_f64 * t13746;
    let t13748 = t676 * t13672;
    let t13750 = t89 * t27 * t13748;
    let t13753 = t89 * t375 * t3822;
    (t13740, t13743, t13746, t13747, t13750, t13753)
}
