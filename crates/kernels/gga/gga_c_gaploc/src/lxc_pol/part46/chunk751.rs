//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 751/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk751<F: Float>(t29853: F, t6508: F, t122: F, t2310: F, t481: F, t158: F, t9127: F, t3085: F, t447: F, t475: F) -> (F, F, F, F, F, F) {
    let t29854 = t6508 * t29853;
    let t29874 = t481 * t2310 * t122;
    let t29882 = t158 * t9127;
    let t29969 = t3085 * t447;
    let t29970 = t6508 * t29969;
    let t29975 = t3085 * t475;
    (t29854, t29874, t29882, t29969, t29970, t29975)
}
