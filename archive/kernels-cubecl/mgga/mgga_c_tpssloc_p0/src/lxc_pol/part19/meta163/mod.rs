//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta163 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk783;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk784;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta163<F: Float>(t185: F, t9449: F, t138: F, t2409: F, t125: F, t2412: F, t701: F, t2414: F, t2379: F, t776: F) -> (F, F, F, F, F, F, F) {
        let (t9450, t9452, t9453, t9454, t9455, t9457) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk783::<F>(t185, t9449, t138, t2409, t125, t2412, t701, t2414);
        let t9458 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk784::<F>(t2379, t776);
    (t9450, t9452, t9453, t9454, t9455, t9457, t9458)
}
