//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 15/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk15<F: Float>(t21: F, t17: F, t19: F, t18: F, t9: F) -> (F, F, F, F) {
    let t22 = pow_1_4::<F>(F::cast_from(4.0_f64));
    let t23 = t22 * t22;
    let t24 = t23 * t22;
    let t25 = t21 * t24;
    let t26 = pow_1_4::<F>(t17);
    let t30 = F::exp(-F::cast_from(0.25916439866088_f64) * t19);
    let t34 = F::cast_from(0.538074483500437_f64) - F::cast_from(0.5565237477462975_f64) * t25 * t26 + F::cast_from(0.6549274647407946_f64) * t30 * t9 * t18;
    (t24, t26, t30, t34)
}
