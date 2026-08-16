//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1168/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1168(t20356: f64, t1499: f64, t7035: f64, t16931: f64, t16783: f64, t16787: f64, t16906: f64, t16909: f64, t16915: f64, t16923: f64, t20346: f64, t20348: f64, t20349: f64, t20350: f64, t20351: f64, t20352: f64, t20354: f64) -> (f64, f64, f64, f64) {
    let t20357 = 0.17544670867903938621e1_f64 * t20356;
    let t20358 = t7035 * t1499;
    let t20359 = 0.17544670867903938621e1_f64 * t20358;
    let t20360 = 48.0_f64 * t16931;
    let t20361 = t16783 - t16787 - t20346 - t16906 + t16909 - t20348 + t20349 + t16915 - t20350 + t20351 - t16923 - t20352 - t20354 - t20357 - t20359 - t20360;
    (t20357, t20359, t20360, t20361)
}
