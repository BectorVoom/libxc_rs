//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta269 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1219;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1220;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1221;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta269(t533: f64, t6995: f64, t1390: f64, t1983: f64, t1388: f64, t3701: f64, t2019: f64, t113: f64, t1266: f64, t1393: f64, t1869: f64, t1976: f64, t1980: f64, t510: f64, t574: f64, t650: f64, t6515: f64, t6517: f64, t652: f64, t6522: f64, t6524: f64, t6527: f64, t6537: f64, t6539: f64, t672: f64, t6862: f64, t6872: f64, t6877: f64, t6882: f64, t3: f64, t112: f64, t2022: f64, t1873: f64, t3938: f64, t671: f64, t3941: f64, t1401: f64, t6534: f64, t577: f64, t1184: f64, t460: f64, t33: f64, t3953: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6996, t6997, t6999, t7000, t7002) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1219(t533, t6995, t1390, t1983, t1388, t3701, t2019, t113, t1266, t1393, t1869, t1976, t1980, t510, t574, t650, t6515, t6517, t652, t6522, t6524, t6527, t6537, t6539, t672, t6862, t6872, t6877, t6882);
        let (t7003, t7010) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1220(t3, t7002, t112, t2022);
        let (t7015, t7020, t7319, t7428) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1221(t1873, t3938, t671, t3941, t1401, t6534, t577, t7002, t7010, t1184, t460, t33, t3953);
    (t6996, t6997, t6999, t7000, t7002, t7003, t7010, t7015, t7020, t7319, t7428)
}
