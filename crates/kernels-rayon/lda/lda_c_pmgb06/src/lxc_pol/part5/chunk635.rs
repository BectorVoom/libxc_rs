//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 635/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk635(t165: f64, t511: f64, t1447: f64, t1989: f64, t1886: f64, t224: f64) -> (f64, f64, f64) {
    let t5179 = t165 * t511;
    let t5186 = 4.0_f64 / 135.0_f64 * t1447 * t1989;
    let t5187 = t1886 * t224;
    (t5179, t5186, t5187)
}
