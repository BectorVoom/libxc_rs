//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 657/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk657<F: Float>(t33274: F, t766: F, t1454: F, t713: F, t6008: F, t193: F, t170: F, t24447: F) -> (F, F, F, F, F) {
    let t33275 = t33274 * t766;
    let t33277 = t1454 * t713;
    let t33278 = t6008 * t33277;
    let t33279 = t193 * t33278;
    let t33282 = t24447 * t170;
    (t33275, t33277, t33278, t33279, t33282)
}
