//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 670/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk670<F: Float>(t1457: F, t285: F, t545: F, t1368: F, t762: F, t147: F, t366: F) -> (F, F, F) {
    let t5690 = t1457 * t545 * t285;
    let t5694 = 0.87170224553660758101e-3 * t762 * t1368 * t285;
    let t5697 = t366 * t147;
    (t5690, t5694, t5697)
}
