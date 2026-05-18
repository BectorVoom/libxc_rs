//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 135/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk135<F: Float>(t118: F, t383: F, t120: F, t30: F, t31: F, t123: F, t126: F) -> (F, F, F, F, F) {
    let t528 = F::new(1.0) / t118;
    let t529 = t528 * t383;
    let t530 = t529 * t120;
    let t532 = t31 * t30;
    let t533 = F::new(1.0) / t532;
    let t534 = t123 * t533;
    let t535 = t383 * t126;
    let t538 = F::new(0.23410285231011484e0) * t530 - F::new(0.532971647967385935e-1) * t534 * t535;
    (t528, t529, t532, t534, t538)
}
