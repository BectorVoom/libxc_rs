//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 150/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk150<F: Float>(t44: F, t415: F, t472: F, t53: F, zeta_threshold: F) -> (F, F) {
    let t45 = t44 <= zeta_threshold;
    let t473 = t472 * t415;
    let t475 = piecewise3::<F>(t45, F::new(0.0), F::new(2.0) / F::new(3.0) * t473);
    let t476 = F::new(1.0) / t53;
    (t475, t476)
}
