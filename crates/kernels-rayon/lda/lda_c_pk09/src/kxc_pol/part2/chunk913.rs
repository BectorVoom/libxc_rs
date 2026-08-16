//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 913/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk913(t4886: f64, t9695: f64, t2: f64, t271: f64, t1197: f64, t258: f64, t4895: f64, t620: f64, t2459: f64, t1193: f64, t1195: f64, t2455: f64, t2460: f64, t4882: f64, t9680: f64, t9689: f64, t9692: f64) -> (f64, f64, f64, f64) {
    let t9696 = t9695 * t4886;
    let t9699 = t271 * t2;
    let t9700 = t258 * t1197;
    let t9701 = t9699 * t9700;
    let t9704 = t4895 * t620;
    let t9705 = t2459 * t9704;
    let t9708 = t9680 * t1193 + t2455 * t620 * t1197 + t9689 - t9692 + 1.28_f64 * t4882 * t2460 - 1.28_f64 * t1195 * t9696 + 2.56_f64 * t1195 * t9701 - 1.28_f64 * t1195 * t9705;
    (t9699, t9700, t9704, t9708)
}
