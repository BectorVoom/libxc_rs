//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 908/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk908(t1091: f64, t2619: f64, t724: f64, t1882: f64, t3844: f64, t3821: f64, t713: f64, t2574: f64, t265: f64, t766: f64, t729: f64, t762: f64) -> (f64, f64, f64, f64) {
    let t14048 = t724 * t2619 * t1091;
    let t14052 = 2.0_f64 / 9.0_f64 * t1882 * t3844;
    let t14053 = t3821 * t713;
    let t14055 = t2574 * t265 * t14053;
    let t14058 = t3821 * t766;
    let t14060 = t729 * t762 * t14058;
    (t14048, t14052, t14055, t14060)
}
