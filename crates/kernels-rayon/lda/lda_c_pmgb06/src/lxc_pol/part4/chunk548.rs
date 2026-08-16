//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 548/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk548(t2311: f64, t388: f64, t118: f64, t1795: f64, t415: f64, t795: f64, t409: f64, t794: f64, t419: f64, t421: f64, t117: f64, t123: f64, t315: f64, t859: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2312 = t388 * t2311;
    let t2323 = t1795 * t118;
    let t2327 = t795 * t415;
    let t2329 = t409 * t794;
    let t2331 = t2329 * t419 * t421;
    let t2338 = t123 * t315 * t859 * t117;
    (t2312, t2323, t2327, t2329, t2331, t2338)
}
