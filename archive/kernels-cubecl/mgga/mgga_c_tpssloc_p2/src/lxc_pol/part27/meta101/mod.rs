//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta101 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk655;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk656;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta101<F: Float>(t2244: F, t2267: F, t2250: F, t43: F, t54: F, t55: F, t240: F, t59: F, t2262: F, t39: F, t44: F, t51: F, t615: F, t618: F, t33: F, t40: F, t632: F, t73: F, t52: F, t636: F, t76: F, t634: F, t638: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2274, t2275, t2278, t2281, t2282, t2283) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk655::<F>(t2244, t2267, t2250, t43, t54, t55, t240, t59, t2262, t39, t44, t51, t615, t618);
        let (t2284, t2289, t2291, t2296, t2298, t2303) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk656::<F>(t2283, t33, t40, t632, t73, t52, t636, t76, t2244, t2250, t634, t638);
    (t2274, t2275, t2278, t2281, t2282, t2283, t2284, t2289, t2291, t2296, t2298, t2303)
}
