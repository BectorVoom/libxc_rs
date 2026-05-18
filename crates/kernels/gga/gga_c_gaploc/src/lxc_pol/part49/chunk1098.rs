//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1098/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1098<F: Float>(t41586: F, t42470: F, t42473: F, t42475: F, t42481: F, t42483: F, t42485: F, t42917: F, t43346: F, t47070: F, t47071: F, t3749: F, t7822: F) -> (F, F) {
    let t47095 = F::new(2.0) * t42917 - t47070 + t41586 + t42470 + t47071 + t42473 + t43346 - t42475 + t42481 - t42483 + t42485;
    let t47096 = t7822 * t3749;
    (t47095, t47096)
}
