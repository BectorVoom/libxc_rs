//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2608/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2608(t10477: f64, t1742: f64, t11713: f64, t3503: f64, t1210: f64, t11719: f64, t13969: f64, t15626: f64, t11529: f64, t1174: f64, t4729: f64, t11647: f64, t1731: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53081 = t1742 * t10477;
    let t53083 = t11713 * t3503 * t53081;
    let t53087 = t11713 * t1210 * t53081;
    let t53093 = t11719 * t13969 * t15626;
    let t53096 = t1174 * t11529 * t4729;
    let t53099 = t1731 * t11647;
    (t53081, t53083, t53087, t53093, t53096, t53099)
}
