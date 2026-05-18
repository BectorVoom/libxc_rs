//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 745/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk745<F: Float>(t2291: F, t748: F, t155: F, t3121: F, t3123: F, t3131: F, t3132: F, t3149: F, t3165: F, t3173: F, t3177: F, t3191: F, t7691: F, t7694: F, t7706: F) -> F {
    let t7709 = t748 * t2291;
    let t7714 = -F::new(14.71989892086604) * t3121 - F::new(14.71989892086604) * t3123 - t3131 + F::new(0.027433775686566395) * t3132 - F::new(1.8805371096875316) * t7691 - F::new(1.2536914064583544) * t7694 + F::new(2.9824072957409817) * t3149 + t3165 - F::new(19.489173774580152) * t155 * t7706 + F::new(0.027433775686566395) * t7709 - F::new(3.600163427964126) * t3173 + F::new(3.600163427964126) * t3177 - F::new(3.600163427964126) * t3191;
    t7714
}
