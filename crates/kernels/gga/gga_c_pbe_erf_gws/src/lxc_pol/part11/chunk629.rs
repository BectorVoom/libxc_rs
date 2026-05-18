//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 629/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk629<F: Float>(t5559: F, t5560: F, t1464: F, t242: F, t366: F, t5: F) -> (F, F, F) {
    let t5562 = F::new(0.15154381759259259259e-2) * t5559 * t5560;
    let t5588 = F::new(0.50257692321302641125e0) * t1464 * t242;
    let t5589 = t5 * t366;
    (t5562, t5588, t5589)
}
