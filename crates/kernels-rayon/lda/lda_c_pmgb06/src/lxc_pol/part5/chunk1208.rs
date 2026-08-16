//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1208/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1208(t21720: f64, t21729: f64, t21774: f64, t21781: f64, t21796: f64, t21803: f64, t21812: f64, t21824: f64, t11758: f64, t19209: f64, t19211: f64, t19215: f64, t19217: f64, t19219: f64, t19221: f64, t19224: f64, t19227: f64, t19231: f64, t19233: f64, t19236: f64) -> (f64, f64) {
    let t21827 = t21720 + t21729 + t21774 + t21781 + t21796 + t21803 + t21812 + t21824;
    let t21850 = -t19209 + t19211 - t19215 - t19217 + t19219 + t19221 + t11758 - t19224 - t19227 - t19231 - t19233 + t19236;
    (t21827, t21850)
}
