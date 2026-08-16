//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta147 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk774;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk775;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta147<F: Float>(t3237: F, t3238: F, t3245: F, t3250: F, t3254: F, t423: F, t1094: F, t1098: F, t1119: F, t1097: F, t419: F, t409: F, t1117: F) -> (F, F, F, F, F, F, F) {
        let (t3256, t3258, t3259, t3261, t3263, t3264) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk774::<F>(t3237, t3238, t3245, t3250, t3254, t423, t1094, t1098, t1119, t1097, t419, t409);
        let t3265 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk775::<F>(t1117);
    (t3256, t3258, t3259, t3261, t3263, t3264, t3265)
}
