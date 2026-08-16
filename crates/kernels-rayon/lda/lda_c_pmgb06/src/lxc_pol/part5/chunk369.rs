//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 369/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk369(t122: f64, t1669: f64, t227: f64, t569: f64, t610: f64, t574: f64, t591: f64, t581: f64, t110: f64, t211: f64) -> (f64, f64, f64, f64, f64) {
    let t1672 = 0.053059442957798957_f64 * t122 * t1669 * t227;
    let t1674 = t122 * t569 * t610;
    let t1676 = t574 * t591;
    let t1679 = 4.0_f64 / 9.0_f64 * t581 * t591;
    let t1680 = t211 * t110;
    (t1672, t1674, t1676, t1679, t1680)
}
