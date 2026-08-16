//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1094/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1094(t11455: f64, t11996: f64, t12000: f64, t12003: f64, t12007: f64, t12009: f64, t12011: f64, t12014: f64, t12018: f64, t12020: f64, t12023: f64, t12026: f64, t12028: f64, t12030: f64, t1805: f64, t1847: f64, t2752: f64, t455: f64, t6907: f64, t6911: f64, t6924: f64, t7489: f64) -> f64 {
    let t12036 = 0.04115066352984959_f64 * t11996 + 3.7610742193750633_f64 * t7489 * t2752 - 19.489173774580152_f64 * t12000 * t1805 + 19.489173774580152_f64 * t12003 * t455 - 4.937333717448355_f64 * t12007 - 4.738783832122567_f64 * t12009 - 1.8805371096875316_f64 * t12011 * t1805 + 1.8805371096875316_f64 * t12014 * t455 - 1.8805371096875316_f64 * t12018 - 4.937333717448355_f64 * t12020 * t1805 + 4.937333717448355_f64 * t12023 * t455 - 0.8091720650647759_f64 * t12026 + 4.738783832122567_f64 * t12028 + 0.8091720650647759_f64 * t12030 + 7.108175748183851_f64 * t1847 * t11455 + 1.1846959580306418_f64 * t6907 - 4.738783832122567_f64 * t6911 + t6924;
    t12036
}
