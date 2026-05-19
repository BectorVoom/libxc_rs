//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1141/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1141<F: Float>(t44051: F, t44053: F, t44057: F, t44060: F, t44064: F, t44065: F, t44069: F, t44070: F, t44074: F, t44076: F, t44079: F, t44083: F) -> F {
    let t47482 = -t44051 - F::cast_from(0.62115540045351614476e2_f64) * t44053 + t44057 + t44060 - t44064 + F::cast_from(0.10725146985555128001e1_f64) * t44065 + t44069 - F::cast_from(0.29792074959875355558e-1_f64) * t44070 - t44074 + F::cast_from(0.69017266717057349418e1_f64) * t44076 + t44079 - t44083;
    t47482
}
