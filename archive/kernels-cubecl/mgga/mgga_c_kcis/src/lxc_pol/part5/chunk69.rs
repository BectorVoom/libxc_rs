//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 69/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk69<F: Float>(t169: F, t171: F, t37: F, t168: F, zeta_threshold: F) -> (F, F) {
    let t170 = t169 <= zeta_threshold;
    let t173 = piecewise3::<F>(t170, t37, t171 * t169);
    let t174 = F::cast_from(1.0_f64) - t168;
    (t173, t174)
}
