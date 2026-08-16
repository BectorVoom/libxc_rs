//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1027/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1027(t1453: f64, t22470: f64, t1982: f64, t8944: f64, t22751: f64, t7692: f64, t1834: f64, t214: f64) -> (f64, f64, f64, f64) {
    let t26127 = t22470 * t1453;
    let t26161 = t1982 * t8944;
    let t26184 = t22751 * t7692;
    let t26193 = t214 * t1834;
    (t26127, t26161, t26184, t26193)
}
