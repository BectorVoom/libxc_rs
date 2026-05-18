//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1094/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1094<F: Float>(t1382: F, t13836: F, t605: F, t39340: F, t921: F, t41586: F, t42470: F, t42473: F, t42475: F, t42481: F, t42483: F, t42485: F, t42487: F, t42491: F, t42494: F) -> (F, F, F) {
    let t47070 = F::new(2.0) * t1382 * t13836 * t605;
    let t47071 = t39340 * t921;
    let t47072 = t47070 - t41586 - t42470 - t47071 - t42473 + t42475 - t42481 + t42483 - t42485 + t42487 + t42491 + t42494;
    (t47070, t47071, t47072)
}
