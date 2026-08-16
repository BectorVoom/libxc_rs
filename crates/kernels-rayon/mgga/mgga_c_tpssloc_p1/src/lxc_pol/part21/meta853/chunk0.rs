//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3082/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3082(t136: f64, t43761: f64, t63420: f64, t3297: f64, t63311: f64, t63315: f64, t63368: f64, t11219: f64, t63372: f64, t63378: f64, t1113: f64, t63402: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t63918 = t136 * t43761 * t63420;
    let t63921 = t136 * t3297 * t63311;
    let t63924 = t136 * t3297 * t63315;
    let t63927 = t136 * t3297 * t63368;
    let t63930 = t136 * t11219 * t63372;
    let t63933 = t136 * t11219 * t63378;
    let t63936 = t136 * t1113 * t63402;
    (t63918, t63921, t63924, t63927, t63930, t63933, t63936)
}
