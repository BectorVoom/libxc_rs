//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1260/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1260(t26727: f64, t2927: f64, t1128: f64, t9761: f64, t2881: f64, t2868: f64, t10154: f64, t10516: f64, t11261: f64, t11263: f64, t11265: f64, t8586: f64, t8951: f64, t8953: f64, t9315: f64, t9316: f64, t9412: f64, t9413: f64, t9415: f64) -> (f64, f64, f64, f64, f64) {
    let t26886 = t2927 * t26727;
    let t26927 = t9761 * t1128;
    let t26973 = t2881 * t26727;
    let t26976 = t2868 * t26727;
    let t27002 = 2.0_f64 * t10154 + 2.0_f64 * t10516 + 2.0_f64 * t9315 + 2.0_f64 * t9316 + 2.0_f64 * t9412 + 4.0_f64 * t8951 + 2.0_f64 * t8953 + 4.0_f64 * t9413 + 2.0_f64 * t9415 + 4.0_f64 * t11263 + 2.0_f64 * t11265 + 2.0_f64 * t8586 + 4.0_f64 * t11261;
    (t26886, t26927, t26973, t26976, t27002)
}
