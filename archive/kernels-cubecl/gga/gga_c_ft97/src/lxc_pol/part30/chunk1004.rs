//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1004/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1004<F: Float>(t10157: F, t35353: F, t446: F, t713: F, t35323: F, t18: F, t2354: F, t3281: F, t33476: F, t150049: F, t24432: F, t24437: F) -> (F, F, F, F) {
    let t150154 = t446 * t10157 * t35353 * t713;
    let t150158 = t446 * t10157 * t35323 * t713;
    let t150162 = t3281 * t2354 * t33476 * t18;
    let t150165 = t24437 * t24432 * t150049;
    (t150154, t150158, t150162, t150165)
}
