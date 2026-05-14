//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 831/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk831<F: Float>(t140919: F, t679: F, t24286: F, t7470: F, t6815: F, t3789: F, t39: F, t40: F, t41547: F, t13519: F, t17836: F, t24287: F, t7453: F, t15: F, t33435: F) -> (F, F, F, F, F, F, F) {
    let t140920 = t140919 * t679;
    let t140927 = t7470 * t24286;
    let t140929 = 0.75685073759570552987e-4 * t6815 * t140927;
    let t140932 = t3789 * t41547 * t39 * t40;
    let t140937 = t17836 * t13519;
    let t140941 = 0.17024962234567901235e-1 * t7453 * t24287;
    let t140943 = t33435 * t15;
    (t140920, t140927, t140929, t140932, t140937, t140941, t140943)
}
