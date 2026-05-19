//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 850/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk850<F: Float>(t387: F, t5882: F, t113: F, t301: F, t4463: F, t1798: F, t413: F, t297: F, t1183: F, t794: F, t123: F, t1309: F, t1316: F, t2180: F, t315: F, t317: F, t342: F, t346: F, t388: F, t4006: F, t4021: F, t4030: F, t4034: F, t4575: F, t5601: F, t5705: F, t5718: F, t5721: F, t5731: F, t5737: F, t61: F, t73: F, t790: F) -> (F, F, F, F, F, F, F) {
    let t5883 = t5882 * t387;
    let t5887 = t4463 * t113 * t301;
    let t5891 = t1798 * t413 * t301;
    let t5893 = F::cast_from(0.02394846802050922_f64) * t297 * t5891;
    let t5895 = t794 * t1183 * t301;
    let t5896 = t297 * t5895;
    let t5898 = F::cast_from(0.39633663517353707_f64) * t4030 - t5601 - F::cast_from(0.054045904796391424_f64) * t4034 + F::cast_from(0.020267214298646783_f64) * t123 * t315 * t4575 * t317 + (t5705 + t5718) * t61 + F::new(6.0) * t1316 * t388 * t5721 + F::new(6.0) * t1316 * t790 * t4006 + F::new(6.0) * t2180 * t790 * t4021 + F::new(12.0) * t2180 * t5731 * t342 + t346 * t790 * t1309 + F::new(6.0) * t2180 * t5737 + t346 * t5883 * t73 - F::cast_from(0.01197423401025461_f64) * t297 * t5887 - t5893 - F::cast_from(0.01197423401025461_f64) * t5896;
    (t5883, t5887, t5891, t5893, t5895, t5896, t5898)
}
