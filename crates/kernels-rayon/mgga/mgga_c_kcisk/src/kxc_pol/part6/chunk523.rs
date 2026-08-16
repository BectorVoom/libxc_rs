//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 523/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk523(t1308: f64, t6458: f64, t2321: f64, t3973: f64, t1580: f64, t2327: f64, t4419: f64, t535: f64, t2326: f64, t4374: f64, t1528: f64, t2285: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6459 = t6458 * t1308;
    let t6473 = t3973 * t2321;
    let t6474 = t1580 * t6473;
    let t6497 = t4419 * t2327;
    let t6498 = t535 * t6497;
    let t6505 = t4374 * t2326;
    let t6518 = t2285 * t1528;
    (t6459, t6473, t6474, t6497, t6498, t6505, t6518)
}
