//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 851/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk851<F: Float>(t1767: F, t794: F, t419: F, t1770: F, t4042: F, t789: F, t346: F, t4045: F, t4242: F, t4245: F, t4249: F, t4296: F, t4301: F, t4302: F, t4304: F, t4307: F, t4314: F, t4318: F, t4322: F, t4324: F, t4325: F) -> (F, F, F, F, F) {
    let t5899 = t1767 * t794;
    let t5900 = t5899 * t419;
    let t5901 = t5900 * t1770;
    let t5903 = t789 * t4042;
    let t5913 = -F::cast_from(1.82185769317151e-05_f64) * t5901 + F::cast_from(2.0_f64) * t346 * t5903 * t4045 + t4242 - F::cast_from(3.64371538634302e-05_f64) * t4245 - t4249 - t4296 - t4301 + F::cast_from(0.019957056683757683_f64) * t4302 + F::cast_from(0.07982822673503073_f64) * t4304 + t4307 - F::cast_from(0.01197423401025461_f64) * t4314 - F::cast_from(0.02394846802050922_f64) * t4318 + t4322 - t4324 - F::cast_from(0.10643763564670763_f64) * t4325;
    (t5899, t5900, t5901, t5903, t5913)
}
