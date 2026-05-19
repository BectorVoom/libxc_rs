//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 274/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk274<F: Float>(t1228: F, t1232: F, t1235: F, t215: F, t584: F, t596: F, t600: F, t606: F) -> F {
    let t1240 = F::cast_from(0.028458728544442837_f64) * t1228 * t584 * t215 - F::cast_from(0.13318739042300334_f64) * t1232 * t596 + F::cast_from(0.004023984722077967_f64) * t600 * t1235 - F::cast_from(0.008569245379942334_f64) * t606 * t1235;
    t1240
}
