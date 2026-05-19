//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1126/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1126<F: Float>(t43716: F, t43719: F, t43721: F, t43723: F, t43726: F, t43729: F, t43731: F, t43735: F, t43737: F, t43740: F, t43743: F, t43746: F) -> F {
    let t47354 = -t43716 + t43719 + t43721 + t43723 + t43726 + t43729 + F::cast_from(0.71500979903700853338e0_f64) * t43731 - t43735 + t43737 - t43740 - t43743 - t43746;
    t47354
}
