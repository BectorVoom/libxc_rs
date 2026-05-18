//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 403/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk403<F: Float>(t3295: F, t969: F, t825: F, t3209: F, t836: F, t568: F, t3234: F, t808: F, t3191: F, t325: F, t3190: F, t813: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3296 = t969 * t3295;
    let t3297 = t825 * t3296;
    let t3298 = F::new(0.38342925953920749676e0) * t3297;
    let t3299 = t836 * t3209;
    let t3300 = t568 * t3299;
    let t3303 = t808 * t3234;
    let t3304 = t568 * t3303;
    let t3307 = t3191 * t325;
    let t3308 = t3190 * t3307;
    let t3309 = t813 * t3308;
    (t3296, t3297, t3298, t3299, t3300, t3303, t3304, t3307, t3308, t3309)
}
