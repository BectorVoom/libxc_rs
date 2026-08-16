//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1033/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1033(t11175: f64, t11176: f64, t1740: f64, t309: f64, t454: f64, t2730: f64, t902: f64, t633: f64, t93: f64, t1792: f64, t2747: f64, t1240: f64) -> (f64, f64, f64, f64, f64) {
    let t11177 = t11175 * t11176;
    let t11179 = t309 * t454 * t1740;
    let t11182 = t902 * t2730;
    let t11183 = t11182 * t633;
    let t11184 = t93 * t11183;
    let t11187 = t2747 * t1792;
    let t11191 = t2747 * t1240;
    (t11177, t11179, t11184, t11187, t11191)
}
