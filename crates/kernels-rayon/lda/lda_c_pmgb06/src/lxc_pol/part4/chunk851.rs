//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 851/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk851(t1767: f64, t794: f64, t419: f64, t1770: f64, t4042: f64, t789: f64, t346: f64, t4045: f64, t4242: f64, t4245: f64, t4249: f64, t4296: f64, t4301: f64, t4302: f64, t4304: f64, t4307: f64, t4314: f64, t4318: f64, t4322: f64, t4324: f64, t4325: f64) -> (f64, f64, f64, f64, f64) {
    let t5899 = t1767 * t794;
    let t5900 = t5899 * t419;
    let t5901 = t5900 * t1770;
    let t5903 = t789 * t4042;
    let t5913 = -1.82185769317151e-05_f64 * t5901 + 2.0_f64 * t346 * t5903 * t4045 + t4242 - 3.64371538634302e-05_f64 * t4245 - t4249 - t4296 - t4301 + 0.019957056683757683_f64 * t4302 + 0.07982822673503073_f64 * t4304 + t4307 - 0.01197423401025461_f64 * t4314 - 0.02394846802050922_f64 * t4318 + t4322 - t4324 - 0.10643763564670763_f64 * t4325;
    (t5899, t5900, t5901, t5903, t5913)
}
