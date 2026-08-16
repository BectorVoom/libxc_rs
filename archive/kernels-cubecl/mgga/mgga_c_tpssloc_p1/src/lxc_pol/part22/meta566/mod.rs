//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta566 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2071;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2072;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta566<F: Float>(t10402: F, t11037: F, t2402: F, t973: F, t999: F, t1030: F, t10477: F, t10472: F, t10475: F, t3128: F, t10969: F, t121: F, t1043: F, t204: F, t1041: F, t248: F, t884: F, t10337: F, t964: F, t340: F, t625: F, t221: F, t339: F, t344: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t42546, t42552, t42559, t42561, t42565, t42592) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2071::<F>(t10402, t11037, t2402, t973, t999, t1030, t10477, t10472, t10475, t3128, t10969, t121);
        let (t42749, t42752, t42811, t42813, t42817) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2072::<F>(t1043, t204, t1041, t248, t884, t10337, t964, t340, t625, t221, t339, t344);
    (t42546, t42552, t42559, t42561, t42565, t42592, t42749, t42752, t42811, t42813, t42817)
}
