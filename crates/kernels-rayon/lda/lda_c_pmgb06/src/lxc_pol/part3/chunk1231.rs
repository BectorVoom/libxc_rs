//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1231/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1231(t2209: f64, t384: f64, t123: f64, t317: f64, t4575: f64, t740: f64, t10599: f64, t10603: f64, t10606: f64, t10609: f64, t10614: f64, t10617: f64, t10620: f64, t10623: f64, t10635: f64, t10640: f64, t10643: f64, t10646: f64, t1316: f64, t2258: f64, t388: f64, t4006: f64) -> f64 {
    let t14617 = t384 * t2209;
    let t14623 = t123 * t740 * t4575 * t317;
    let t14625 = t10599 - t10603 + 0.004067943812504169_f64 * t10606 + 0.012203831437512505_f64 * t10609 - t10614 + t10617 - 0.0002905674151788692_f64 * t10620 - 0.0017434044910732151_f64 * t10623 - 0.002615106736609823_f64 * t10635 + t10640 - t10643 - 0.020146007452401596_f64 * t10646 + 18.0_f64 * t1316 * t2258 * t4006 + 9.0_f64 * t1316 * t388 * t14617 - 0.16213771438917426_f64 * t14623;
    t14625
}
