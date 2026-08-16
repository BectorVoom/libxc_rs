//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta638 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2054;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2055;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta638<F: Float>(t87729: F, t25325: F, t6547: F, t1911: F, t40889: F, t23185: F, t25045: F, t82074: F, t225: F, t25161: F, t6562: F, t6572: F, t86893: F, t23171: F, t23228: F, t7488: F, t214: F, t4265: F, t25055: F, t81591: F, t25217: F, t25060: F, t25222: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t87730, t87734, t87748, t87754, t87758, t87776) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2054::<F>(t87729, t25325, t6547, t1911, t40889, t23185, t25045, t82074, t225, t25161, t6562, t6572, t86893);
        let (t87777, t87779, t87782, t87787, t87797, t87805, t87810) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2055::<F>(t87776, t23171, t23228, t7488, t214, t4265, t25055, t81591, t25217, t6547, t25060, t225, t25222);
    (t87730, t87734, t87748, t87754, t87758, t87777, t87779, t87782, t87787, t87797, t87805, t87810)
}
