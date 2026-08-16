//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta111 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk675;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk676;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta111<F: Float>(t205: F, t2570: F, t786: F, t792: F, t118: F, t776: F, t794: F, t59: F, t835: F, t154: F, t116: F, t206: F, t212: F, t225: F, t799: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t2571, t2576, t2578, t2579, t2585, t2586) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk675::<F>(t205, t2570, t786, t792, t118, t776, t794, t59, t835, t154);
        let (t2587, t2588, t2590, t2597) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk676::<F>(t116, t206, t212, t2586, t225, t799);
    (t2571, t2576, t2578, t2579, t2585, t2586, t2587, t2588, t2590, t2597)
}
