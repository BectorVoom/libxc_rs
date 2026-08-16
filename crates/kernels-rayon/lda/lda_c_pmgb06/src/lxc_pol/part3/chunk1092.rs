//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1092/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1092(t3098: f64, t465: f64, t1069: f64, t477: f64, t760: f64, t5083: f64, t12514: f64, t495: f64, t5065: f64, t5072: f64, t2970: f64, t5077: f64, t823: f64) -> (f64, f64, f64, f64, f64) {
    let t13000 = t465 * t3098;
    let t13002 = t760 * t1069 * t477;
    let t13005 = 2.0_f64 / 3.0_f64 * t5083 * t13000 * t13002;
    let t13007 = t5065 * t12514 * t495;
    let t13008 = t13007 * t5072;
    let t13009 = 8.0_f64 / 45.0_f64 * t13008;
    let t13012 = 2.0_f64 / 15.0_f64 * t5077 * t823 * t2970;
    (t13002, t13005, t13007, t13009, t13012)
}
