//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2282/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2282(t1134: f64, t16857: f64, t3399: f64, t5071: f64, t3407: f64, t5079: f64, t5087: f64, t5101: f64, t698: f64, t1145: f64, t16746: f64, t141: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16858 = t16857 * t1134;
    let t16860 = t5071 * t3399;
    let t16862 = t3407 * t5079;
    let t16863 = t16862 * t1134;
    let t16865 = t5087 * t3399;
    let t16868 = t698 * t5101;
    let t16869 = 0.10954222222222222222e0_f64 * t16868;
    let t16870 = t1145 * t16746;
    let t16871 = t141 * t16870;
    (t16858, t16860, t16863, t16865, t16868, t16869, t16870, t16871)
}
