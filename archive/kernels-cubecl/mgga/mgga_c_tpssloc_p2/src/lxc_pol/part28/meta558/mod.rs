//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta558 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1829;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1830;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta558<F: Float>(t1404: F, t7222: F, t24447: F, t580: F, t2098: F, t3946: F, t1395: F, t7240: F, t1453: F, t81439: F, t26129: F, t81442: F, t22470: F, t4067: F, t2332: F, t81446: F, t666: F, t22473: F, t2358: F, t12808: F, t6530: F, t12816: F, t191: F, t192: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t85381, t85392, t85394, t85397, t86586, t86588) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1829::<F>(t1404, t7222, t24447, t580, t2098, t3946, t1395, t7240, t1453, t81439, t26129, t81442);
        let (t86590, t86593, t86596, t86599, t86601, t86672) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1830::<F>(t22470, t4067, t1453, t2332, t81446, t666, t22473, t2358, t12808, t6530, t12816, t191, t192);
    (t85381, t85392, t85394, t85397, t86586, t86588, t86590, t86593, t86596, t86599, t86601, t86672)
}
