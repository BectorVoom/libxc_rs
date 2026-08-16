//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1207/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1207(t10687: f64, t11479: f64, t3275: f64, t11514: f64, t1554: f64, t3579: f64, t10831: f64, t1102: f64, t3692: f64, t1543: f64, t3582: f64, t10610: f64, t3276: f64) -> (f64, f64, f64, f64) {
    let t40479 = t3275 * t11479 * t10687 / 4.0_f64;
    let t40483 = t3579 * t1554 * t11514 / 4.0_f64;
    let t40485 = t1102 * t10831 * t3692;
    let t40487 = t3582 * t1543;
    let t40490 = 15.0_f64 / 8.0_f64 * t10610 * t3276 * t40487;
    (t40479, t40483, t40485, t40490)
}
