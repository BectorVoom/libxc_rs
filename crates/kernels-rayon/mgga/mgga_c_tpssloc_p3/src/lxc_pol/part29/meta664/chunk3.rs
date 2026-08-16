//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2209/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2209(t5187: f64, t562: f64, t1352: f64, t22633: f64, t6976: f64, t1799: f64, t6637: f64, t6888: f64, t81129: f64, t22881: f64, t16049: f64, t1992: f64, t81027: f64) -> (f64, f64, f64, f64, f64) {
    let t90818 = t562 * t5187;
    let t90821 = t22633 * t6976 * t90818 * t1352;
    let t90825 = t6888 * t6637 * t81129 * t1799;
    let t90829 = t6888 * t6637 * t22881 * t5187;
    let t90832 = t1992 * t81027 * t16049;
    (t90818, t90821, t90825, t90829, t90832)
}
