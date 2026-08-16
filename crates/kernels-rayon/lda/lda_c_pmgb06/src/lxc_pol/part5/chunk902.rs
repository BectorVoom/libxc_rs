//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 902/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk902(t2778: f64, t415: f64, t2791: f64, t399: f64, t10524: f64, t117: f64, t84: f64, t118: f64, t3993: f64, t391: f64, t1329: f64, t1347: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10847 = 0.0004746123948660562_f64 * t2778 * t415;
    let t10848 = t399 * t2791;
    let t10852 = 0.031505407223141116_f64 * t84 * t10524 * t117;
    let t10857 = t3993 * t118;
    let t10860 = 0.12602162889256446_f64 * t391 * t2791;
    let t10861 = t1329 * t1347;
    (t10847, t10848, t10852, t10857, t10860, t10861)
}
