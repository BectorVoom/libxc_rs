//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 307/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk307(t12: f64, t430: f64, t453: f64, t987: f64, t995: f64, t87: f64, t972: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t998 = 0.165625e-1_f64 * t430 * t987 - 0.165625e-1_f64 * t453 * t995;
    let t1002 = piecewise3(t84, 0.0_f64, 4.0_f64 / 3.0_f64 * t87 * t972);
    let t1003 = -t972;
    (t998, t1002, t1003)
}
