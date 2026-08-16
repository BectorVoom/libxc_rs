//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1047/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1047(t2569: f64, t3848: f64, t1436: f64, t8890: f64, t2551: f64, t3883: f64, t903: f64, t1449: f64, t2613: f64, t2595: f64, t3886: f64, t2621: f64, t3882: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11379 = t3848 * t2569;
    let t11382 = t1436 * t8890;
    let t11383 = t11382 * t2551;
    let t11390 = t3883 * t903;
    let t11393 = t1449 * t2613;
    let t11396 = t3886 * t2595;
    let t11399 = t3882 * t2621;
    (t11379, t11383, t11390, t11393, t11396, t11399)
}
