//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1196;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1197;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1198;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta257(t533: f64, t6995: f64, t1390: f64, t1983: f64, t1388: f64, t3701: f64, t2019: f64, t1873: f64, t3938: f64, t671: f64, t3941: f64, t1401: f64, t6534: f64, t2108: f64, t33: f64, t2240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6996, t6997, t6998, t6999, t7000, t7001, t7014, t7015, t7017, t7019) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1196(t533, t6995, t1390, t1983, t1388, t3701, t2019, t1873, t3938, t671, t3941, t1401, t6534);
        let t7245 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1197(t2108, t33);
        let t7246 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1198(t2240, t7245);
    (t6996, t6997, t6998, t6999, t7000, t7001, t7014, t7015, t7017, t7019, t7245, t7246)
}
