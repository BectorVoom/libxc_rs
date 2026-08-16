//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta200 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1016;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta200<F: Float>(t4493: F, t959: F, t1580: F, t2929: F, t2932: F, t950: F, t1592: F, t2970: F, t973: F, t2978: F, t60: F, t344: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4495, t4496, t4497, t4498, t4500, t4506, t4507, t4509, t4510) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1016::<F>(t4493, t959, t1580, t2929, t2932, t950, t1592, t2970, t973, t2978, t60, t344);
    (t4495, t4496, t4497, t4498, t4500, t4506, t4507, t4509, t4510)
}
