//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 580/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk580(t265: f64, t3746: f64, t724: f64, t1175: f64, t684: f64, t1168: f64, t713: f64, t729: f64, t762: f64, t766: f64, t2568: f64, t242: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3852 = t724 * t265 * t3746;
    let t3856 = t724 * t1175 * t684;
    let t3859 = t1168 * t713;
    let t3861 = t729 * t762 * t3859;
    let t3864 = t1168 * t766;
    let t3865 = t2568 * t3864;
    let t3866 = t242 * t3865;
    (t3852, t3856, t3859, t3861, t3864, t3865, t3866)
}
