//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 500/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk500(t1868: f64, t473: f64, t103: f64, t1523: f64, t1607: f64, t1614: f64, t1615: f64, t1856: f64, t1861: f64, t1866: f64, t1870: f64, t2052: f64, t2054: f64, t2057: f64, t2060: f64) -> (f64, f64) {
    let t2061 = t473 * t1868;
    let t2064 = t1607 + 0.011997222222222222_f64 * t1523 + 0.011997222222222222_f64 * t1856 - 0.023994444444444443_f64 * t1861 + 0.07198333333333333_f64 * t1866 - 0.07198333333333333_f64 * t1870 + t1614 + 0.0044444444444444444_f64 * t1615 + 0.0044444444444444444_f64 * t2052 - 0.0022222222222222222_f64 * t103 * t2054 + 0.013333333333333334_f64 * t103 * t2057 - 0.013333333333333334_f64 * t2060 * t2061;
    (t2061, t2064)
}
