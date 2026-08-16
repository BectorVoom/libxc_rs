//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 978/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk978(t1366: f64, t7193: f64, t5102: f64, t831: f64, t432: f64, t6626: f64, t13715: f64, t153: f64, t2501: f64, t3213: f64, t464: f64, t6123: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16446 = t7193 * t1366;
    let t16448 = t831 * t5102;
    let t16455 = t432 * t6626;
    let t16491 = t13715 * t153;
    let t16506 = t3213 * t2501;
    let t16513 = t6123 * t464;
    (t16446, t16448, t16455, t16491, t16506, t16513)
}
