//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 274/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk274(t1228: f64, t1232: f64, t1235: f64, t215: f64, t584: f64, t596: f64, t600: f64, t606: f64) -> f64 {
    let t1240 = 0.028458728544442837_f64 * t1228 * t584 * t215 - 0.13318739042300334_f64 * t1232 * t596 + 0.004023984722077967_f64 * t600 * t1235 - 0.008569245379942334_f64 * t606 * t1235;
    t1240
}
