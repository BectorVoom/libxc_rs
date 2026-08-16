//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta503 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2140;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta503<F: Float>(t17161: F, t2826: F, t136: F, t10304: F, t17152: F, t17167: F, t908: F, t17171: F, t17183: F, t17178: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17240, t17241, t17243, t17244, t17246, t17247, t17249, t17250, t17252, t17253, t17255, t17256) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2140::<F>(t17161, t2826, t136, t10304, t17152, t17167, t908, t17171, t17183, t17178);
    (t17240, t17241, t17243, t17244, t17246, t17247, t17249, t17250, t17252, t17253, t17255, t17256)
}
