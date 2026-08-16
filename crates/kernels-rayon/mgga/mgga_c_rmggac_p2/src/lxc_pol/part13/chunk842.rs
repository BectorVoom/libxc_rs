//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 842/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk842(t7498: f64, t8659: f64, t7505: f64, t8365: f64, t1971: f64, t2144: f64, t495: f64, t5898: f64, t7230: f64, t5267: f64, t3351: f64, t498: f64, t7231: f64) -> (f64, f64, f64, f64, f64) {
    let t38899 = t8659 * t7498;
    let t38901 = t8365 * t7505;
    let t38908 = t7230 * t1971 * t2144 * t5898 * t495;
    let t38913 = t7230 * t1971 * t2144 * t5267 * t495;
    let t38918 = t3351 * t7231 * t2144 * t5267 * t498;
    (t38899, t38901, t38908, t38913, t38918)
}
