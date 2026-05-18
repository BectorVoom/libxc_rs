//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 770/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk770<F: Float>(t729: F, t7484: F, t773: F, t1882: F, t7508: F, t242: F, t33275: F, t33272: F, t7553: F, t761: F, t684: F, t2606: F) -> (F, F, F, F, F, F, F) {
    let t33743 = t729 * t773 * t7484;
    let t33747 = t1882 * t7508 / F::new(9.0);
    let t33748 = t242 * t33275;
    let t33751 = t242 * t33272;
    let t33754 = t761 * t7553;
    let t33755 = t33754 * t684;
    let t33756 = t2606 * t33755;
    (t33743, t33747, t33748, t33751, t33754, t33755, t33756)
}
