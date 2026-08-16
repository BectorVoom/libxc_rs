//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 850/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk850(t387: f64, t5882: f64, t113: f64, t301: f64, t4463: f64, t1798: f64, t413: f64, t297: f64, t1183: f64, t794: f64, t123: f64, t1309: f64, t1316: f64, t2180: f64, t315: f64, t317: f64, t342: f64, t346: f64, t388: f64, t4006: f64, t4021: f64, t4030: f64, t4034: f64, t4575: f64, t5601: f64, t5705: f64, t5718: f64, t5721: f64, t5731: f64, t5737: f64, t61: f64, t73: f64, t790: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5883 = t5882 * t387;
    let t5887 = t4463 * t113 * t301;
    let t5891 = t1798 * t413 * t301;
    let t5893 = 0.02394846802050922_f64 * t297 * t5891;
    let t5895 = t794 * t1183 * t301;
    let t5896 = t297 * t5895;
    let t5898 = 0.39633663517353707_f64 * t4030 - t5601 - 0.054045904796391424_f64 * t4034 + 0.020267214298646783_f64 * t123 * t315 * t4575 * t317 + (t5705 + t5718) * t61 + 6.0_f64 * t1316 * t388 * t5721 + 6.0_f64 * t1316 * t790 * t4006 + 6.0_f64 * t2180 * t790 * t4021 + 12.0_f64 * t2180 * t5731 * t342 + t346 * t790 * t1309 + 6.0_f64 * t2180 * t5737 + t346 * t5883 * t73 - 0.01197423401025461_f64 * t297 * t5887 - t5893 - 0.01197423401025461_f64 * t5896;
    (t5883, t5887, t5891, t5893, t5895, t5896, t5898)
}
