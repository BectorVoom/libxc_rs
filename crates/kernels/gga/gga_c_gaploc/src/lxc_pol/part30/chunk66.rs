//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 66/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk66<F: Float>(t122: F, t158: F, t169: F, t172: F, t105: F, t33: F, t58: F) -> (F, F, F) {
    let t174 = t122 * t158 * t169 * t172;
    let t177 = -t33 + t58 + F::cast_from(0.28455006635676149599e-1_f64) * t105 * t174;
    let t178 = F::sqrt(F::new(4.0));
    (t174, t177, t178)
}
