//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2542/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2542(t52035: f64, t52037: f64, t11223: f64, t1678: f64, t1041: f64, t11262: f64, t4868: f64, t3201: f64, t4794: f64, t4798: f64, t343: f64, t44: f64, t816: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t53252 = 0.39511111111111111112e-1_f64 * t52035;
    let t53253 = 0.13170370370370370371e-1_f64 * t52037;
    let t53281 = t11223 * t1678;
    let t53293 = t1041 * t11262 * t4868;
    let t53294 = 0.14291339372689912324e-3_f64 * t53293;
    let t53300 = t4794 * t3201;
    let t53317 = t4798 * t3201;
    let t53318 = 0.14291339372689912324e-3_f64 * t53317;
    let t53320 = t44 * t343 * t816;
    (t53252, t53253, t53281, t53294, t53300, t53318, t53320)
}
