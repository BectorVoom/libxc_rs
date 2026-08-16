//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta150 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk844;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk845;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta150<F: Float>(t3236: F, t407: F, t3271: F, t1107: F, t3279: F, t281: F, t2820: F, t415: F, t1114: F, t699: F, t1176: F, t241: F) -> (F, F, F, F, F, F, F, F) {
        let (t3282, t3287, t3288, t3290, t3293, t3294, t3295) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk844::<F>(t3236, t407, t3271, t1107, t3279, t281, t2820, t415, t1114, t699);
        let t3297 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk845::<F>(t1176, t241);
    (t3282, t3287, t3288, t3290, t3293, t3294, t3295, t3297)
}
