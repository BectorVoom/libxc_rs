//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1165/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1165(t126: f64, t19684: f64, t83: f64, t16876: f64, t16878: f64, t496: f64, t7024: f64, t501: f64, t6825: f64, t1545: f64, t2607: f64, t16880: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20329 = t83 * t19684 * t126;
    let t20330 = 24.0_f64 * t16876;
    let t20331 = 4.0_f64 * t16878;
    let t20332 = t496 * t7024;
    let t20333 = 12.0_f64 * t20332;
    let t20334 = t501 * t6825;
    let t20335 = 12.0_f64 * t20334;
    let t20336 = t1545 * t2607;
    let t20337 = 36.0_f64 * t20336;
    let t20338 = 480.0_f64 * t16880;
    (t20329, t20330, t20331, t20333, t20335, t20337, t20338)
}
