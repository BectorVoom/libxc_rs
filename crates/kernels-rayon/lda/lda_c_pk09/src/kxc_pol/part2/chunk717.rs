//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 717/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk717(t6522: f64, t6319: f64, t6325: f64, t6547: f64, t6464: f64, t1672: f64, t2071: f64, t2115: f64, t747: f64, t2114: f64, t2111: f64, t2085: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7188 = 1.02153520341379_f64 * t6522;
    let t7192 = 0.15282509383508946_f64 * t6319;
    let t7199 = 0.10188339589005964_f64 * t6325;
    let t7200 = 0.08512793361781583_f64 * t6547;
    let t7205 = 0.033961131963353215_f64 * t6464;
    let t7221 = t2071 * t1672;
    let t7223 = t747 * t2115;
    let t7224 = t2114 * t7223;
    let t7226 = t2111 * t1672;
    let t7228 = t2085 * t1672;
    (t7188, t7192, t7199, t7200, t7205, t7221, t7223, t7224, t7226, t7228)
}
