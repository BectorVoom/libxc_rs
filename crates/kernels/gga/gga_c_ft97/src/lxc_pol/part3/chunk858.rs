//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 858/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk858<F: Float>(t17295: F, t17346: F, t579: F, t91: F, t16925: F, t16928: F, t16739: F, t16742: F, t16756: F, t16760: F, t16922: F, t17249: F, t17250: F, t17251: F) -> (F, F) {
    let t17347 = t17295 + t17346;
    let t17349 = t91 * t579 * t17347;
    let t17351 = t16925 / F::new(3.0);
    let t17352 = F::new(2.0) / F::new(3.0) * t16928;
    let t17353 = -F::new(6.0) * t16739 + F::new(4.0) * t16742 + t17249 - t17250 + t17251 + F::new(2.0) * t16756 - t16760 / F::new(3.0) + t17349 / F::new(2.0) - t16922 + t17351 - t17352;
    (t17349, t17353)
}
