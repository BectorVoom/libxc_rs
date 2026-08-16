//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1090/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1090(t466: f64, t779: f64, t2104: f64, t2107: f64, t5974: f64, t5979: f64, t5589: f64, t735: f64, t154: f64, t276: f64, t277: f64, t4932: f64) -> (f64, f64, f64, f64, f64) {
    let t17867 = t466 * t779;
    let t17869 = t2104 * t17867 * t2107;
    let t17872 = t2104 * t5974 * t5979;
    let t17874 = t735 * t5589;
    let t17881 = 5.0_f64 / 486.0_f64 * t276 * t154 * t4932 * t277;
    (t17867, t17869, t17872, t17874, t17881)
}
