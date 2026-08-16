//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1916/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1916(t1307: f64, t6637: f64, t6888: f64, t90809: f64, t1352: f64, t22633: f64, t6976: f64, t90754: f64, t5187: f64, t562: f64, t1799: f64, t81129: f64) -> (f64, f64, f64, f64, f64) {
    let t90812 = t6888 * t6637 * t90809 * t1307;
    let t90816 = t22633 * t6976 * t90754 * t1352;
    let t90818 = t562 * t5187;
    let t90821 = t22633 * t6976 * t90818 * t1352;
    let t90825 = t6888 * t6637 * t81129 * t1799;
    (t90812, t90816, t90818, t90821, t90825)
}
