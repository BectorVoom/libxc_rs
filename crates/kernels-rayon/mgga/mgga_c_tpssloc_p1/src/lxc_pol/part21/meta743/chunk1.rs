//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2611/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2611(t1222: f64, t15765: f64, t3242: f64, t3448: f64, t11728: f64, t13969: f64, t15630: f64, t11718: f64, t52835: f64, t11797: f64, t5024: f64, t11147: f64, t15394: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53185 = t15765 * t1222;
    let t53187 = t3448 * t3242;
    let t53220 = t11728 * t13969 * t15630;
    let t53238 = t52835 * t11718;
    let t53246 = t5024 * t11797;
    let t53249 = t15394 * t11147;
    (t53185, t53187, t53220, t53238, t53246, t53249)
}
