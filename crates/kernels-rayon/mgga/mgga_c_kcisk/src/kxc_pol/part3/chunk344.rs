//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 344/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk344(t1224: f64, t1636: f64, t1697: f64, t1696: f64, t617: f64, t608: f64, t609: f64, t1695: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1699 = t1224 * t1697 * t1636;
    let t1701 = -t1696 - 0.17808333333333333333e-1_f64 * t1699;
    let t1704 = t617 * t617;
    let t1705 = 1.0_f64 / t1704;
    let t1706 = t608 * t1705;
    let t1707 = 1.0_f64 / t609;
    let t1709 = -t1695 / 3.0_f64 - t1699 / 3.0_f64;
    (t1699, t1701, t1704, t1705, t1706, t1707, t1709)
}
