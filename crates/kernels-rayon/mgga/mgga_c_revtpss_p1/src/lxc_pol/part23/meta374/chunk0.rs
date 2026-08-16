//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1705/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1705(t11249: f64, t1668: f64, t12160: f64, t4891: f64, t1086: f64, t4746: f64, t3090: f64) -> (f64, f64, f64, f64) {
    let t15907 = t1668 * t11249;
    let t15917 = t12160 * t4891;
    let t15925 = t4746 * t1086;
    let t15926 = t15925 * t3090;
    (t15907, t15917, t15925, t15926)
}
