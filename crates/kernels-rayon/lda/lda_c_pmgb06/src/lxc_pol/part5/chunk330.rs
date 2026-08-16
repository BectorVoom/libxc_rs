//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 330/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk330(t410: f64, t56: f64, t69: f64, t1260: f64, t26: f64, t329: f64) -> (f64, f64, f64) {
    let t1302 = 0.3831677777777778_f64 * t69 * t410 * t56;
    let t1303 = t69 * t1260;
    let t1316 = t329 * t26;
    (t1302, t1303, t1316)
}
