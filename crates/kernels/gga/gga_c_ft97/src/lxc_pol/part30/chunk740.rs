//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 740/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk740<F: Float>(t33414: F, t6056: F, t677: F, t6783: F, t27519: F, t32237: F, t3771: F) -> (F, F, F, F) {
    let t33415 = t33414 * t6056;
    let t33418 = t677 * t6783;
    let t33423 = t27519 * t32237;
    let t33424 = t3771 * t33423;
    (t33415, t33418, t33423, t33424)
}
