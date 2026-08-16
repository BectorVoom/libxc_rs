//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta157 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk826;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk827;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk828;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk829;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk830;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk831;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk832;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk833;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta157<F: Float>(t2701: F, t4255: F, t820: F, t4119: F, t847: F, t1516: F, t2621: F, t2623: F, t2640: F, t2643: F, t2695: F, t2698: F, t4191: F, t4236: F, t4240: F, t4250: F, t4253: F, t817: F, t843: F, t4189: F, t218: F, t1520: F, t225: F, t1527: F, t865: F, t2718: F, t2627: F, t68: F, t226: F, t1509: F, t252: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t4257 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk826::<F>(t2701, t4255, t820);
        let t4261 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk827::<F>(t4119, t820, t847);
        let t4264 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk828::<F>(t1516, t2621, t2623, t2640, t2643, t2695, t2698, t4191, t4236, t4240, t4250, t4253, t4257, t4261, t817, t843);
        let t4265 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk829::<F>(t4189, t4264);
        let (t4266, t4268) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk830::<F>(t218, t4265, t1520, t225);
        let (t4272, t4273) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk831::<F>(t1527, t865, t2718);
        let (t4280, t4281) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk832::<F>(t2627, t68, t226);
        let t4282 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk833::<F>(t1509, t252);
    (t4257, t4261, t4265, t4266, t4268, t4272, t4273, t4280, t4281, t4282)
}
