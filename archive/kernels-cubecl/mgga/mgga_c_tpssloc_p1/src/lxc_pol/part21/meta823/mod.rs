//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta823 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2893;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2894;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta823<F: Float>(t14379: F, t49226: F, t2791: F, t5689: F, t2794: F, t4433: F, t2792: F, t2836: F, t5727: F, t10661: F, t17520: F, t2793: F, t2842: F, t10704: F, t5726: F, t10702: F, t13654: F, t4399: F, t17527: F, t42100: F, t42102: F, t5694: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t60354, t60359, t60360, t60371, t60374) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2893::<F>(t14379, t49226, t2791, t5689, t2794, t4433, t2792, t2836, t5727, t10661, t17520, t2793);
        let (t60377, t60381, t60384, t60387, t60391) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2894::<F>(t17520, t2836, t2842, t10704, t5726, t10702, t2793, t13654, t4399, t17527, t42100, t42102, t5694);
    (t60354, t60359, t60360, t60371, t60374, t60377, t60381, t60384, t60387, t60391)
}
