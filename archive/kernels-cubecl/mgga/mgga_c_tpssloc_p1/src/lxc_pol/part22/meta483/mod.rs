//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta483 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1894;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1895;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1896;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta483<F: Float>(t10216: F, t20234: F, t10304: F, t136: F, t20217: F, t883: F, t908: F, t2770: F) -> (F, F, F, F, F, F, F) {
        let t21130 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1894::<F>(t10216, t20234);
        let (t21131, t21132, t21134) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1895::<F>(t10304, t21130, t136, t20217, t883);
        let (t21135, t21136, t21138) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1896::<F>(t21134, t908, t136, t20234, t2770);
    (t21130, t21131, t21132, t21134, t21135, t21136, t21138)
}
