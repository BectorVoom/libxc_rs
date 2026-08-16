//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 940/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk940(t2787: f64, t5771: f64, t2783: f64, t683: f64, t1855: f64, t1084: f64, t1893: f64, t1856: f64, t2786: f64, t5776: f64, t1901: f64, t2782: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7268 = 0.32163958997385070134e2_f64 * t5771 * t2787;
    let t7269 = t2783 * t683;
    let t7271 = 4.0_f64 * t1855 * t7269;
    let t7272 = t1084 * t1893;
    let t7274 = 2.0_f64 * t1855 * t7272;
    let t7275 = t2786 * t1856;
    let t7277 = 0.96491876992155210402e2_f64 * t5776 * t7275;
    let t7278 = t2782 * t1901;
    (t7268, t7269, t7271, t7272, t7274, t7275, t7277, t7278)
}
