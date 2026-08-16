//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta600 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1987;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1988;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta600<F: Float>(t1887: F, t23069: F, t22690: F, t23153: F, t23171: F, t6561: F, t80741: F, t6643: F, t23025: F, t23030: F, t23012: F, t6653: F, t22641: F, t2588: F) -> (F, F, F, F, F, F, F) {
        let t81591 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1987::<F>(t1887, t23069);
        let (t81595, t81597, t81599, t81600, t81602, t81612) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1988::<F>(t22690, t23153, t23171, t6561, t80741, t6643, t23025, t23030, t23012, t6653, t22641, t2588);
    (t81591, t81595, t81597, t81599, t81600, t81602, t81612)
}
