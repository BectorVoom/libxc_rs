//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1243/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1243(t1614: f64, t2967: f64, t1626: f64, t2986: f64, t4587: f64, t914: f64, t936: f64, t2919: f64, t4590: f64, t1596: f64, t2923: f64, t2927: f64) -> (f64, f64, f64, f64, f64) {
    let t15406 = t1614 * t2967;
    let t15413 = t1626 * t2986;
    let t15416 = t4587 * t914;
    let t15418 = 2.0_f64 * t15416 * t936;
    let t15420 = 1.0_f64 * t4590 * t2919;
    let t15421 = t1596 * t2923;
    let t15423 = 0.16081979498692535067e2_f64 * t15421 * t2927;
    (t15406, t15413, t15418, t15420, t15423)
}
