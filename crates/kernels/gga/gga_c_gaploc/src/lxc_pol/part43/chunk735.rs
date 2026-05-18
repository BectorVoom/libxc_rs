//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 735/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk735<F: Float>(t12849: F, t12850: F, t12851: F, t12853: F, t12858: F, t12864: F, t14454: F, t14455: F, t14456: F, t14457: F, t14480: F, t14519: F) -> F {
    let t14520 = -t14454 + t12851 + t14455 - t12853 + t14456 - t12849 + t12858 - t14457 + t12850 - t12864 + t14480;
    let t14521 = t14519 + t14520;
    t14521
}
