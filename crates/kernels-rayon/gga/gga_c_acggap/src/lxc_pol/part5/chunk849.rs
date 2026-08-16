//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 849/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk849(t2981: f64, t883: f64, t11795: f64, t11797: f64, t11800: f64, t11803: f64, t11806: f64, t11811: f64, t11813: f64, t11815: f64, t11817: f64, t11820: f64) -> (f64, f64) {
    let t11922 = t883 * t2981;
    let t11934 = -0.28769444444444444444e1_f64 * t11795 + 0.27618666666666666667e2_f64 * t11797 - 0.10229135802469135803e2_f64 * t11800 + 0.89504938271604938273e1_f64 * t11803 + 0.31310740740740740741e1_f64 * t11806 + 0.366775e-1_f64 * t11811 - 0.58684e0_f64 * t11813 + 0.65204444444444444445e0_f64 * t11815 + 0.5705388888888888889e0_f64 * t11817 + 0.13490888888888888889e1_f64 * t11820;
    (t11922, t11934)
}
