//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta269 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1219;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1220;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1221;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta269<F: Float>(t533: F, t6995: F, t1390: F, t1983: F, t1388: F, t3701: F, t2019: F, t113: F, t1266: F, t1393: F, t1869: F, t1976: F, t1980: F, t510: F, t574: F, t650: F, t6515: F, t6517: F, t652: F, t6522: F, t6524: F, t6527: F, t6537: F, t6539: F, t672: F, t6862: F, t6872: F, t6877: F, t6882: F, t3: F, t112: F, t2022: F, t1873: F, t3938: F, t671: F, t3941: F, t1401: F, t6534: F, t577: F, t1184: F, t460: F, t33: F, t3953: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t6996, t6997, t6999, t7000, t7002) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1219::<F>(t533, t6995, t1390, t1983, t1388, t3701, t2019, t113, t1266, t1393, t1869, t1976, t1980, t510, t574, t650, t6515, t6517, t652, t6522, t6524, t6527, t6537, t6539, t672, t6862, t6872, t6877, t6882);
        let (t7003, t7010) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1220::<F>(t3, t7002, t112, t2022);
        let (t7015, t7020, t7319, t7428) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1221::<F>(t1873, t3938, t671, t3941, t1401, t6534, t577, t7002, t7010, t1184, t460, t33, t3953);
    (t6996, t6997, t6999, t7000, t7002, t7003, t7010, t7015, t7020, t7319, t7428)
}
