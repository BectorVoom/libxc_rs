//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1094/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1094(t132: f64, t2851: f64, t823: f64, t1512: f64, t2015: f64, t432: f64, t5302: f64, t495: f64, t5415: f64, t224: f64, t5431: f64, t1423: f64, t4609: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13090 = t132 * t2851 * t823;
    let t13092 = t1512 * t2015;
    let t13094 = t432 * t5302;
    let t13100 = t495 * t5415;
    let t13104 = t5431 * t224;
    let t13117 = t1423 * t4609;
    (t13090, t13092, t13094, t13100, t13104, t13117)
}
