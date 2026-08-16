//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1115/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1115<F: Float>(t143163: F, t152717: F, t33820: F, t10248: F, t152669: F, t446: F, t35985: F, t681: F, t89: F, t2680: F, t35972: F, t193: F, t824: F) -> (F, F, F, F, F) {
    let t152962 = t33820 * t143163 * t152717;
    let t152965 = t446 * t10248 * t152669;
    let t152970 = t89 * t681 * t35985;
    let t152972 = t2680 * t35972;
    let t152975 = t89 * t193 * t152972 * t824;
    (t152962, t152965, t152970, t152972, t152975)
}
