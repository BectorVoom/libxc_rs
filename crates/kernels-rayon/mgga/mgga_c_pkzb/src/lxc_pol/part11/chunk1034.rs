//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1034/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1034(t11445: f64, t154: f64, t6436: f64, t10251: f64, t1167: f64, t758: f64, t3880: f64, t394: f64, t3186: f64, t406: f64, t3898: f64, t3919: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11447 = t154 * t6436 * t11445;
    let t11451 = t10251 * t1167;
    let t11452 = t758 * t11451;
    let t11456 = t394 * t3880;
    let t11457 = t3186 * t11456;
    let t11458 = t406 * t11457;
    let t11461 = t3919 * t3898;
    (t11447, t11451, t11452, t11456, t11457, t11458, t11461)
}
