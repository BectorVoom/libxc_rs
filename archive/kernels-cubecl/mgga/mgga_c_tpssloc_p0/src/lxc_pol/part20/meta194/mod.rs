//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta194 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1179;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1180;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1181;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1182;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1183;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1184;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta194<F: Float>(t1654: F, t690: F, t1409: F, t3242: F, t607: F, t3240: F, t123: F, t3247: F, t1088: F, t1089: F, t3966: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t4721 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1179::<F>(t1654, t690);
        let t4723 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1180::<F>(t1409, t3242);
        let t4724 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1181::<F>(t4723, t607);
        let (t4725, t4726, t4728) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1182::<F>(t3240, t4724, t123, t1409, t3247);
        let t4729 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1183::<F>(t4728, t607);
        let (t4730, t4731, t4733) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1184::<F>(t1088, t4729, t123, t1089, t3966);
    (t4721, t4723, t4724, t4725, t4726, t4728, t4729, t4730, t4731, t4733)
}
