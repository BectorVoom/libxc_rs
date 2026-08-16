//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1663/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1663(t1134: f64, t16862: f64, t3399: f64, t5087: f64, t5101: f64, t698: f64, t1145: f64, t16746: f64, t141: f64, t16712: f64, t1729: f64, t2439: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16863 = t16862 * t1134;
    let t16865 = t5087 * t3399;
    let t16868 = t698 * t5101;
    let t16869 = 0.10954222222222222222e0_f64 * t16868;
    let t16870 = t1145 * t16746;
    let t16871 = t141 * t16870;
    let t16873 = 0.19931111111111111111e0_f64 * t16712;
    let t16876 = t2439 * t1729;
    (t16863, t16865, t16868, t16869, t16871, t16873, t16876)
}
