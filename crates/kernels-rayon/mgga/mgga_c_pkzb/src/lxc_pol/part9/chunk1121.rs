//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1121/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1121(t179: f64, t19155: f64, t2226: f64, t404: f64, t154: f64, t385: f64, t386: f64, t4932: f64, t6185: f64, t921: f64, t466: f64, t931: f64) -> (f64, f64, f64, f64) {
    let t19158 = t404 * t179 * t19155 * t2226;
    let t19163 = 5.0_f64 / 486.0_f64 * t385 * t154 * t4932 * t386;
    let t19166 = t921 * t6185;
    let t19191 = t466 * t931;
    (t19158, t19163, t19166, t19191)
}
