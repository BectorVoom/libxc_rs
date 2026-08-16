//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta435 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1614;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1615;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta435<F: Float>(t23121: F, t281: F, t22690: F, t776: F, t841: F, t2617: F, t6620: F, t849: F, t2703: F, t6621: F, t6619: F, t835: F, t812: F) -> (F, F, F, F, F, F, F, F) {
        let (t23122, t23124, t23125, t23127, t23128, t23130, t23132) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1614::<F>(t23121, t281, t22690, t776, t841, t2617, t6620, t849, t2703, t6621, t6619, t835);
        let t23133 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1615::<F>(t23132, t812);
    (t23122, t23124, t23125, t23127, t23128, t23130, t23132, t23133)
}
