//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 818/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk818(t1799: f64, t415: f64, t1347: f64, t795: f64, t118: f64, t5522: f64, t1795: f64, t117: f64, t123: f64, t125: f64, t2777: f64, t2814: f64, t3474: f64, t3478: f64, t3481: f64, t5543: f64, t5610: f64, t5615: f64, t5620: f64, t5622: f64, t5625: f64, t5627: f64, t5689: f64) -> f64 {
    let t5697 = 0.06301081444628223_f64 * t1799 * t415;
    let t5698 = t795 * t1347;
    let t5701 = 0.06301081444628223_f64 * t5522 * t118;
    let t5702 = t1795 * t415;
    let t5705 = t5610 - 0.04789693604101844_f64 * t3474 + 0.008980675507690957_f64 * t3478 + 0.006584630109636494_f64 * t5615 - t5620 - 0.003950778065781896_f64 * t5622 - 0.0004954275694490498_f64 * t5625 - 0.06301081444628223_f64 * t5627 - 0.005388405304614574_f64 * t123 * t125 * t5689 * t117 - 0.031505407223141116_f64 * t5543 * t118 - t5697 - 0.031505407223141116_f64 * t5698 + t5701 + 0.06301081444628223_f64 * t5702 + t2777 + t3481 + 0.031505407223141116_f64 * t2814;
    t5705
}
