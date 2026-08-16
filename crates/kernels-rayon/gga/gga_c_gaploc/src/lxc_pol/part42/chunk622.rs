//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 622/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk622(t3634: f64, t747: f64, t3638: f64, t841: f64, t1052: f64, t3073: f64, t3684: f64, t11609: f64, t1457: f64, t325: f64, t3601: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11701 = t3634 * t747;
    let t11711 = t3638 * t841;
    let t11714 = t1052 * t3073;
    let t11718 = t3684 * t841;
    let t11721 = t1457 * t11609;
    let t11724 = t325 * t3601;
    (t11701, t11711, t11714, t11718, t11721, t11724)
}
