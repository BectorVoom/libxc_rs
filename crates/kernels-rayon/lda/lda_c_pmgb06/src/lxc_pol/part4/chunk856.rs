//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 856/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk856(t123: f64, t2407: f64, t317: f64, t740: f64, t2414: f64, t395: f64, t113: f64, t301: f64, t2448: f64, t76: f64, t1316: f64, t1317: f64, t2180: f64, t2308: f64, t2733: f64, t2738: f64, t2741: f64, t329: f64, t342: f64, t346: f64, t4398: f64, t4414: f64, t5569: f64, t5573: f64, t5578: f64, t5580: f64, t5721: f64, t5934: f64, t5937: f64, t5939: f64, t5980: f64, t77: f64, t790: f64) -> (f64, f64, f64) {
    let t5986 = t123 * t740 * t2407 * t317;
    let t5988 = t395 * t2414;
    let t5990 = t5988 * t113 * t301;
    let t5992 = t76 * t2448;
    let t5996 = 3.0_f64 * t1316 * t2733 * t1317 - t346 * t2308 * t5934 + 0.019957056683757683_f64 * t5937 + 6.0_f64 * t5939 * t2738 + 6.0_f64 * t1316 * t790 * t5721 + 6.0_f64 * t1316 * t790 * t4414 - t346 * t4398 * t2741 + 0.002711962541669446_f64 * t5569 + 0.39633663517353707_f64 * t5573 - t5578 - 0.0011622696607154768_f64 * t5580 + 3.0_f64 * t329 * t77 * t5980 - 0.054045904796391424_f64 * t5986 - 0.0002905674151788692_f64 * t5990 + 6.0_f64 * t2180 * t5992 * t342;
    (t5988, t5992, t5996)
}
