//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta180 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1112;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1113;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1114;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1115;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1116;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1117;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta180(t2: f64, t265: f64, t584: f64, t1540: f64, t690: f64, t1409: f64, t2770: f64, t607: f64, t2768: f64, t123: f64, t2775: f64, t882: f64, t3966: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4331, t4332, t4335) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1112(t2, t265, t584, t1540, t690);
        let t4337 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1113(t1409, t2770);
        let t4338 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1114(t4337, t607);
        let (t4339, t4340, t4342) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1115(t2768, t4338, t123, t1409, t2775);
        let t4343 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1116(t4342, t607);
        let (t4344, t4345, t4347) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1117(t4343, t882, t123, t3966, t883);
    (t4331, t4332, t4335, t4337, t4338, t4339, t4340, t4342, t4343, t4344, t4345, t4347)
}
