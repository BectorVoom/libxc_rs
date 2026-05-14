//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 632/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk632<F: Float>(t12996: F, t4391: F, t12959: F, t12962: F, t12966: F, t12970: F, t12972: F, t12976: F, t12979: F, t12983: F, t12989: F, t12992: F, t12994: F, t193: F, t557: F, t574: F, t597: F) -> (F,) {
    let t12997 = t4391 * t12996;
    let t12998 = 0.59584149919750711116e-1 * t12997;
    let t12999 = -t12959 + t12962 - 0.38342925953920749676e0 * t12966 - t12970 + 0.35750489951850426669e0 * t12972 * t193 + 0.23005755572352449806e1 * t597 * t12976 - 0.35750489951850426669e0 * t557 * t12979 - 0.23005755572352449806e1 * t574 * t12983 + t12989 + t12992 + 0.38342925953920749676e0 * t12994 + t12998;
    (t12999,)
}
