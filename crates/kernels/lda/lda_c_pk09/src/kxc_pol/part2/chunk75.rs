//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 75/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk75<F: Float>(t66: F, t77: F, t88: F, t142: F) -> (F, F, F, F) {
    let t197 = F::new(9.375) * t66 + F::cast_from(1.2466946262544771_f64) * t77 + F::cast_from(0.146484375_f64);
    let t198 = F::ln(t197);
    let t199 = t198 * t88;
    let t200 = t199 * t142;
    (t197, t198, t199, t200)
}
