//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1121/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1121<F: Float>(t40640: F, t40641: F, t43069: F, t43071: F, t43072: F, t43075: F, t43076: F, t43077: F, t43078: F, t43079: F, t43080: F) -> F {
    let t47311 = t43069 - t43071 + t43072 / F::cast_from(2.0_f64) + t40640 - t40641 + t43075 + t43076 - t43077 + t43078 - t43079 - t43080;
    t47311
}
