//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 910/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk910(t40081: f64, t7720: f64, t7487: f64, t8343: f64, t8358: f64, t8362: f64, t2001: f64, t2281: f64, t326: f64, t333: f64, t495: f64, t515: f64, t7230: f64, t7231: f64, t9109: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40082 = t7720 * t40081;
    let t40084 = t7487 * t8343;
    let t40086 = t7487 * t8358;
    let t40088 = t7487 * t8362;
    let t40092 = t2001 * t326 * t2281 * t333;
    let t40093 = t7720 * t40092;
    let t40098 = t7230 * t7231 * t515 * t9109 * t495;
    (t40082, t40084, t40086, t40088, t40093, t40098)
}
