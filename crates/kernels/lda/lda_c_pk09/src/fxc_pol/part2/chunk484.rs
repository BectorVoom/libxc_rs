//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 484/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk484<F: Float>(t2703: F, t420: F, t1181: F, t1182: F, t1183: F, t1184: F, t1702: F, t1689: F, t1692: F, t253: F) -> (F, F, F, F) {
    let t2704 = t2703 * t420;
    let t2707 = t1181 + t1182 + t1183 + t1184;
    let t2708 = t1702 * t2707;
    let t2711 = t1689 - t1692 + F::new(1.28) * t253 * t2704 - F::new(1.28) * t253 * t2708;
    (t2704, t2707, t2708, t2711)
}
