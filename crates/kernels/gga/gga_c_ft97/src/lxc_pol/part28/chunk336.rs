//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 336/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk336<F: Float>(t5507: F, t5508: F, t28: F, t1293: F, t25: F, t3066: F, t38: F, t401: F) -> (F, F, F, F, F) {
    let t5509 = t5507 * t5508;
    let t5510 = t28 * t5509;
    let t5513 = t1293 * t25;
    let t5514 = t5513 * t3066;
    let t5517 = t38 * t401;
    (t5509, t5510, t5513, t5514, t5517)
}
