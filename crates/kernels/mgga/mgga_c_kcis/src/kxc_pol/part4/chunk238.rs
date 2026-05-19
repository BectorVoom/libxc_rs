//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 238/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk238<F: Float>(t169: F, t171: F, t829: F, zeta_threshold: F) -> (F, F) {
    let t170 = t169 <= zeta_threshold;
    let t832 = piecewise3::<F>(t170, F::new(0.0), F::new(4.0) / F::new(3.0) * t171 * t829);
    let t833 = -t829;
    (t832, t833)
}
