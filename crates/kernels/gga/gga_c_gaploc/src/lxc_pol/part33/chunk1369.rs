//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1369/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1369<F: Float>(t30113: F, t30118: F, t30120: F, t30123: F, t31961: F, t31966: F, t31969: F, t31974: F, t31984: F, t31988: F, t31990: F, t31994: F, t31998: F, t32001: F, t32003: F) -> F {
    let t38385 = t31961 + t31966 + t31969 + t31974 + t31984 + t31988 - t31990 - t31994 - t31998 + t32001 + t32003 - t30113 + t30118 + t30120 + t30123;
    t38385
}
