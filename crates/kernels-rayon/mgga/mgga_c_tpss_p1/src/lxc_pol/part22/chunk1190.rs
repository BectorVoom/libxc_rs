//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1190/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1190(t17974: f64, t803: f64, t2391: f64, t5559: f64, t2395: f64, t1705: f64, t2398: f64, t935: f64, t5567: f64, t5570: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17975 = t17974 * t803;
    let t17976 = 7.0_f64 / 288.0_f64 * t17975;
    let t17977 = t5559 * t2391;
    let t17979 = t5559 * t2395;
    let t17990 = t1705 * t2398;
    let t17991 = t17990 * t935;
    let t17993 = t5567 * t5570;
    (t17975, t17976, t17977, t17979, t17990, t17991, t17993)
}
