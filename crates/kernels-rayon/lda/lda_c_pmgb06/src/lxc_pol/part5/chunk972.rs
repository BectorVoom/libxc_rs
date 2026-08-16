//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 972/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk972(t12036: f64, t835: f64, t2462: f64, t3223: f64, t1435: f64, t2582: f64, t1423: f64, t6556: f64, t13712: f64, t806: f64, t2485: f64, t3213: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16104 = t12036 * t835;
    let t16106 = t3223 * t2462;
    let t16118 = t1435 * t2582;
    let t16137 = t1423 * t6556;
    let t16144 = t13712 * t806;
    let t16150 = t3213 * t2485;
    (t16104, t16106, t16118, t16137, t16144, t16150)
}
