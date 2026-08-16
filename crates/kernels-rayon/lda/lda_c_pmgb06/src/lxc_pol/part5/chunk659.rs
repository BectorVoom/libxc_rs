//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 659/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk659(t1282: f64, t34: f64, t1798: f64, t301: f64, t413: f64, t297: f64, t1183: f64, t794: f64, t1767: f64, t419: f64, t1770: f64, t4042: f64, t789: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5874 = t34 * t1282;
    let t5891 = t1798 * t413 * t301;
    let t5893 = 0.02394846802050922_f64 * t297 * t5891;
    let t5895 = t794 * t1183 * t301;
    let t5896 = t297 * t5895;
    let t5899 = t1767 * t794;
    let t5900 = t5899 * t419;
    let t5901 = t5900 * t1770;
    let t5903 = t789 * t4042;
    (t5874, t5891, t5893, t5895, t5896, t5899, t5900, t5901, t5903)
}
