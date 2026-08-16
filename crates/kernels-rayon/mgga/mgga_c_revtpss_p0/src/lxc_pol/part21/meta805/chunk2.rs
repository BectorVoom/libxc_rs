//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2930/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2930(t3201: f64, t4798: f64, t343: f64, t44: f64, t816: f64, t11821: f64, t65: f64, t11144: f64, t11970: f64, t1660: f64, t27527: f64, t2852: f64) -> (f64, f64, f64, f64, f64) {
    let t53317 = t4798 * t3201;
    let t53318 = 0.14291339372689912324e-3_f64 * t53317;
    let t53320 = t44 * t343 * t816;
    let t53321 = t65 * t11821;
    let t53322 = t53321 * t11144;
    let t53326 = t1660 * t11970;
    let t53328 = t27527 * t2852;
    (t53318, t53320, t53322, t53326, t53328)
}
