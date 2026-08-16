//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta532 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2005;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2006;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta532<F: Float>(t591: F, t9688: F, t2386: F, t240: F, t2385: F, t2558: F, t686: F, t685: F, t120: F, t118: F, t123: F, t116: F, t268: F, t8705: F, t9701: F, t2397: F, t693: F, t119: F, t133: F, t39273: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t39275, t39277, t39278, t39280, t39281, t39283, t39284, t39289) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2005::<F>(t591, t9688, t2386, t240, t2385, t2558, t686, t685, t120, t118, t123, t116, t268, t8705);
        let (t39291, t39293, t39295, t39298, t39300) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2006::<F>(t591, t9701, t2397, t39277, t39280, t693, t119, t133, t240, t39273, t39275, t39278, t39281, t39284, t39289);
    (t39275, t39278, t39281, t39283, t39284, t39289, t39291, t39293, t39295, t39298, t39300)
}
