//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1077/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1077(t5589: f64, t735: f64, t154: f64, t276: f64, t277: f64, t4932: f64, t5612: f64, t771: f64, t178: f64, t299: f64, t301: f64, t4902: f64) -> (f64, f64, f64, f64) {
    let t17874 = t735 * t5589;
    let t17881 = 5.0_f64 / 486.0_f64 * t276 * t154 * t4932 * t277;
    let t17897 = t771 * t5612;
    let t17902 = 0.14820648238345094262e-3_f64 * t299 * t178 * t4902 * t301;
    (t17874, t17881, t17897, t17902)
}
