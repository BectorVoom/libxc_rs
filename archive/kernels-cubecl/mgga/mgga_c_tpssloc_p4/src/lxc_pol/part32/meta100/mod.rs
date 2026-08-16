//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta100 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk641;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk642;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk643;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk644;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk645;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk646;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta100<F: Float>(t2225: F, t594: F, t598: F, t15: F, t19: F, t601: F, t604: F, t84: F, t85: F, t24: F, t42: F, t54: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t2226, t2228, t2229) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk641::<F>(t2225, t594, t598, t15);
        let t2230 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk642::<F>(t2229);
        let (t2232, t2235) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk643::<F>(t19, t2230, t601, t604);
        let t2239 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk644::<F>(t84, t85);
        let t2240 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk645::<F>(t2239, t24);
        let (t2267, t2274) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk646::<F>(t42, t54);
    (t2226, t2228, t2229, t2230, t2232, t2235, t2239, t2240, t2267, t2274)
}
