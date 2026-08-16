//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1007/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1007(t3080: f64, t862: f64, t1189: f64, t2278: f64, t3103: f64, t870: f64, t1197: f64, t2273: f64, t2258: f64, t3106: f64, t2281: f64, t3102: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8115 = t3080 * t862;
    let t8120 = t1189 * t2278;
    let t8129 = t3103 * t870;
    let t8132 = t1197 * t2273;
    let t8135 = t3106 * t2258;
    let t8138 = t3102 * t2281;
    (t8115, t8120, t8129, t8132, t8135, t8138)
}
