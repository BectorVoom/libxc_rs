//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 745/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk745(t2291: f64, t748: f64, t155: f64, t3121: f64, t3123: f64, t3131: f64, t3132: f64, t3149: f64, t3165: f64, t3173: f64, t3177: f64, t3191: f64, t7691: f64, t7694: f64, t7706: f64) -> f64 {
    let t7709 = t748 * t2291;
    let t7714 = -14.71989892086604_f64 * t3121 - 14.71989892086604_f64 * t3123 - t3131 + 0.027433775686566395_f64 * t3132 - 1.8805371096875316_f64 * t7691 - 1.2536914064583544_f64 * t7694 + 2.9824072957409817_f64 * t3149 + t3165 - 19.489173774580152_f64 * t155 * t7706 + 0.027433775686566395_f64 * t7709 - 3.600163427964126_f64 * t3173 + 3.600163427964126_f64 * t3177 - 3.600163427964126_f64 * t3191;
    t7714
}
