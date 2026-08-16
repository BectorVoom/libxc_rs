//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1005/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1005<F: Float>(t140763: F, t150045: F, t33292: F, t150120: F, t24432: F, t6118: F, t24437: F, t24438: F, t27775: F, t33476: F, t35343: F, t684: F) -> (F, F, F, F) {
    let t150168 = t33292 * t140763 * t150045;
    let t150171 = t6118 * t24432 * t150120;
    let t150175 = t24437 * t24438 * t33476 * t27775;
    let t150179 = t24437 * t24438 * t35343 * t684;
    (t150168, t150171, t150175, t150179)
}
