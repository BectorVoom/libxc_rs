//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1417/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1417(t208: f64, t213: f64, t579: f64, t6716: f64, t588: f64, t6717: f64, t97: f64, t1696: f64, t2414: f64, t6721: f64, t15116: f64, t16874: f64, t16876: f64, t16878: f64, t16881: f64, t16883: f64, t16885: f64, t16886: f64, t16891: f64, t16894: f64, t16895: f64, t205: f64) -> f64 {
    let t18274 = t6716 * t579 * t208 * t213;
    let t18277 = t6717 * t97 * t588;
    let t18281 = t2414 * t1696 * t208 * t213;
    let t18284 = t6721 * t97 * t588;
    let t18286 = t16874 + t15116 * t205 * t208 * t213 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t18274 + 0.12155555555555556_f64 * t18277 + t18281 / 3.0_f64 + 0.12155555555555556_f64 * t18284 - t16876 - t16878 + t16881 + t16883 - t16885 - t16886 - t16891 + t16894 + t16895;
    t18286
}
