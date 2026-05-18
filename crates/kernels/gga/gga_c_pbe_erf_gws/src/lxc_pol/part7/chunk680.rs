//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 680/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk680<F: Float>(t5463: F, t644: F, t639: F, t1782: F, t586: F) -> (F, F, F) {
    let t5464 = t5463 * t644;
    let t5465 = t639 * t5464;
    let t5466 = F::new(8.0) / F::new(135.0) * t5465;
    let t5467 = t1782 * t586;
    (t5464, t5466, t5467)
}
