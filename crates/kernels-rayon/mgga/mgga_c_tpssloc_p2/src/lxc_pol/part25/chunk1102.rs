//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1102/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1102(t22827: f64, t3788: f64, t3792: f64, t54770: f64, t1339: f64, t54591: f64, t550: f64, t40197: f64, t54858: f64, t6936: f64, t12392: f64, t6945: f64) -> (f64, f64, f64, f64, f64) {
    let t80974 = t22827 * t3788 * t54770 * t3792;
    let t80978 = t22827 * t1339 * t54591 * t550;
    let t80982 = t22827 * t1339 * t40197 * t550;
    let t80985 = t6936 * t3788 * t54858;
    let t80987 = t6945 * t12392;
    (t80974, t80978, t80982, t80985, t80987)
}
