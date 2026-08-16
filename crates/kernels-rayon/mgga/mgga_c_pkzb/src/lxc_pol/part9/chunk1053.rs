//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1053/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1053(t1769: f64, t5312: f64, t1726: f64, t5389: f64, t5393: f64, t158: f64, t165: f64, t5387: f64, t1721: f64, t5381: f64, t5397: f64, t1760: f64, t5384: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16409 = t1769 * t5312;
    let t16416 = t5389 * t1726;
    let t16417 = t16416 * t5393;
    let t16421 = t158 / t5387 / t165;
    let t16425 = t1721 * t1721;
    let t16438 = t5381 * t5397;
    let t16440 = t5384 * t1760;
    (t16409, t16417, t16421, t16425, t16438, t16440)
}
