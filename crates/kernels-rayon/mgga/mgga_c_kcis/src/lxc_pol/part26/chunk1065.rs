//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1065/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1065(t2237: f64, t27348: f64, t3717: f64, t531: f64, t4142: f64, t7925: f64, t1542: f64, t491: f64) -> (f64, f64, f64, f64) {
    let t27349 = t2237 * t27348;
    let t27356 = t3717 * t531;
    let t27362 = t4142 * t7925;
    let t27364 = t1542 * t491;
    (t27349, t27356, t27362, t27364)
}
