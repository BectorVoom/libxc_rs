//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1383/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1383(t13969: f64, t4599: f64, t3039: f64, t376: f64, t4649: f64, t4594: f64, t4582: f64, t3120: f64, t3131: f64, t4593: f64, t10482: f64, t3040: f64) -> (f64, f64, f64, f64, f64) {
    let t13970 = t13969 * t4599;
    let t13972 = t3039 * t13970 / 2304.0_f64;
    let t13975 = t376 * t4649;
    let t13976 = t13975 * t4594;
    let t13977 = t4582 * t13976;
    let t13980 = t3131 * t3120;
    let t13981 = t4593 * t13980;
    let t13982 = t4582 * t13981;
    let t13985 = t10482 * t3040;
    (t13972, t13975, t13977, t13982, t13985)
}
