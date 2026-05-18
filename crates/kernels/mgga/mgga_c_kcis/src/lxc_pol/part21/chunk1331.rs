//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1331/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1331<F: Float>(t1808: F, t982: F, t1020: F, t7719: F, t27910: F, t93435: F, t7703: F, t14059: F, t2179: F, t303: F, t27924: F, t3317: F) -> (F, F, F, F, F) {
    let t96476 = t1808 * t982;
    let t96478 = t1020 * t96476 * t7719;
    let t96480 = t93435 * t27910;
    let t96482 = F::new(0.46336805555555555556e-3) * t7703 * t96480;
    let t96486 = t303 * t14059 * t2179;
    let t96489 = t303 * t27924 * t3317;
    (t96478, t96480, t96482, t96486, t96489)
}
