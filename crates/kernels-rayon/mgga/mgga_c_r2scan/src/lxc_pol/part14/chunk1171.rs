//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1171/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1171(t122: f64, t3434: f64, t3437: f64, t40453: f64, t1563: f64, t2867: f64, t10831: f64, t1102: f64, t3692: f64, t1543: f64, t3582: f64, t2333: f64, t2526: f64) -> (f64, f64, f64, f64, f64) {
    let t40460 = t3434 * t3437 * t40453 * t122;
    let t40464 = t2867 * t1563;
    let t40485 = t1102 * t10831 * t3692;
    let t40487 = t3582 * t1543;
    let t40491 = t2333 * t2526;
    (t40460, t40464, t40485, t40487, t40491)
}
