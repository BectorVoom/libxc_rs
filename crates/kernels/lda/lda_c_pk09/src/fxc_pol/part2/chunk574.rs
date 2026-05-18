//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 574/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk574<F: Float>(t160: F, t2971: F, t701: F, t183: F, t699: F, t3930: F, t698: F, t655: F, t694: F, t121: F, t120: F, t718: F) -> (F, F, F, F) {
    let t4044 = t160 * t2971;
    let t4049 = t701 * t701;
    let t4050 = F::new(1.0) / t4049;
    let t4053 = t183 * t699;
    let t4056 = F::new(1.0) / t3930;
    let t4057 = t698 * t4056;
    let t4059 = -F::new(2.0) * t4053 * t655 + t4057 * t694;
    let t4060 = t121 * t4059;
    let t4061 = t120 * t4060;
    let t4064 = t718 * t2971;
    (t4044, t4050, t4061, t4064)
}
