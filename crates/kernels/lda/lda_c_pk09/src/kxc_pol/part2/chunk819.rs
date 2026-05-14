//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 819/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk819<F: Float>(t359: F, t9739: F, t355: F, t347: F, t1222: F, t2512: F) -> (F, F, F, F) {
    let t9827 = t359 * t9739;
    let t9830 = t355 * t9739;
    let t9833 = t347 * t9739;
    let t9836 = t1222 * t2512;
    (t9827, t9830, t9833, t9836)
}
