//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1017/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1017(t51: f64, t11007: f64, t11030: f64, t1719: f64, t2719: f64, t425: f64, t2724: f64, t6403: f64, t2723: f64, t4878: f64, t6360: f64, t1701: f64, t2140: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t52 = t51 <= zeta_threshold;
    let t11033 = piecewise3(t52, t11007, t11030 * t425 + t1719 * t2719);
    let t11039 = 1.28_f64 * t6403 * t2724;
    let t11040 = t2723 * t4878;
    let t11042 = 1.28_f64 * t6360 * t11040;
    let t11045 = t1701 * t2140;
    (t11033, t11039, t11042, t11045)
}
