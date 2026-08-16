//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta266 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1421;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1422;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta266<F: Float>(t10316: F, t908: F, t136: F, t10250: F, t883: F, t9258: F, t10295: F, t10296: F, t10298: F, t10300: F, t10302: F, t10307: F, t10311: F, t10314: F) -> (F, F, F, F, F, F, F, F) {
        let (t10317, t10318, t10319, t10320, t10321) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1421::<F>(t10316, t908, t136, t10250, t883, t9258);
        let (t10322, t10323, t10325) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1422::<F>(t10321, t908, t136, t10295, t10296, t10298, t10300, t10302, t10307, t10311, t10314, t10318, t10320);
    (t10317, t10318, t10319, t10320, t10321, t10322, t10323, t10325)
}
