//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1844;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1845;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta567<F: Float>(t25041: F, t87049: F, t215: F, t6581: F, t252: F, t81613: F, t13224: F, t23056: F, t13352: F, t25242: F, t6579: F, t25245: F, t82031: F, t25038: F, t4282: F, t6646: F, t9647: F, t25251: F, t23012: F, t7529: F, t13380: F, t22986: F, t2647: F, t13377: F, t1880: F, t1894: F, t214: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t87050, t87052, t87055, t87059, t87066, t87068) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1844::<F>(t25041, t87049, t215, t6581, t252, t81613, t13224, t23056, t13352, t25242, t6579, t25245, t82031);
        let (t87076, t87078, t87080, t87084, t87092) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1845::<F>(t25038, t4282, t6646, t9647, t25251, t87049, t23012, t7529, t13380, t22986, t2647, t13377, t1880, t1894, t214);
    (t87050, t87052, t87055, t87059, t87066, t87068, t87076, t87078, t87080, t87084, t87092)
}
