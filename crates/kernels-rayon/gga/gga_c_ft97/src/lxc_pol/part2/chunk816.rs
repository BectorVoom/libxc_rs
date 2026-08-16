//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 816/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk816(t150: f64, t358: f64, t378: f64, t3524: f64, t458: f64, t12302: f64, t2102: f64, t11034: f64, t3499: f64, t2: f64, t9224: f64, t11008: f64) -> (f64, f64, f64, f64, f64) {
    let t12812 = t378 * t150 * t358;
    let t12816 = 2.0_f64 / 3.0_f64 * t458 * t3524;
    let t12817 = t2102 * t12302;
    let t12820 = t3499 * t11034;
    let t12823 = t9224 * t2;
    let t12824 = t12823 * t11008;
    (t12812, t12816, t12817, t12820, t12824)
}
