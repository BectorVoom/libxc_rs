//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1159/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1159(t26494: f64, t7642: f64, t209: f64, t2155: f64, t7645: f64, t8915: f64, t2398: f64, t26477: f64, t8944: f64, t26430: f64, t7647: f64, t7639: f64) -> (f64, f64, f64, f64, f64) {
    let t92082 = t7642 * t26494;
    let t92086 = t2155 * t209 * t7645 * t8915;
    let t92089 = t8944 * t2398 * t26477;
    let t92091 = t26430 * t7647;
    let t92093 = t26430 * t7639;
    (t92082, t92086, t92089, t92091, t92093)
}
