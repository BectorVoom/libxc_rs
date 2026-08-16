//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1412/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1412(t5760: f64, t9292: f64, t40921: f64, t5737: f64, t4101: f64, t5740: f64, t9288: f64, t40270: f64, t1892: f64, t9990: f64, t1897: f64, t40317: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t49172 = t9292 * t5760;
    let t49178 = t40921 * t5737;
    let t49203 = t4101 * t5740 * t9288;
    let t49210 = t40270 * t5737;
    let t49327 = t9990 * t1892;
    let t49354 = t40317 * t1897;
    (t49172, t49178, t49203, t49210, t49327, t49354)
}
