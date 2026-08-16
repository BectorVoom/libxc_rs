//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1216/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1216<F: Float>(t12846: F, t12849: F, t12850: F, t12851: F, t12853: F, t12854: F, t12858: F, t12864: F, t13761: F, t13762: F, t13763: F, t13764: F, t13767: F, t13837: F, t13954: F, t47073: F, t48248: F, t48250: F, t7: F) -> F {
    let tv4rhosigma34 = t13761 - t13762 - t12851 + t12853 - t12854 - t12846 + t12849 - t12850 - t12858 + t13763 + t12864 - t13764 + t13767 - t13837 + t13954 + t7 * (t47073 + t48248 + t48250);
    tv4rhosigma34
}
