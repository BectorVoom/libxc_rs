//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1119/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1119(t2395: f64, t2397: f64, t5939: f64, t17928: f64, t395: f64, t6512: f64, t326: f64, t2370: f64, t6513: f64, t17955: f64, t918: f64, t922: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19102 = t2395 * t5939 * t2397;
    let t19106 = t17928 / t6512 / t395;
    let t19107 = t19106 * t326;
    let t19109 = t2370 * t2370;
    let t19115 = t17928 * t6513;
    let t19116 = t19115 * t326;
    let t19124 = t918 * t17955 * t922;
    (t19102, t19106, t19107, t19109, t19115, t19116, t19124)
}
