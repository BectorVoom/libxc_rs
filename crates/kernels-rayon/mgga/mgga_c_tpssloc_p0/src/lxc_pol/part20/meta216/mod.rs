//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta216 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1270;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1271;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1272;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1273;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1274;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1275;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1276;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1277;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta216(t5286: f64, t550: f64, t1343: f64, t820: f64, t1352: f64, t5248: f64, t5249: f64, t120: f64, t1799: f64, t3805: f64, t1831: f64, t3866: f64, t1307: f64, t3870: f64, t1367: f64, t5187: f64, t1341: f64, t1363: f64, t3781: f64, t3783: f64, t3800: f64, t3803: f64, t3864: f64, t3867: f64, t5259: f64, t5257: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5287, t5289) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1270(t5286, t550, t1343, t820);
        let t5293 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1271(t1352, t5248, t5249);
        let t5301 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1272(t120, t1799);
        let t5303 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1273(t1352, t3805, t5301);
        let (t5306, t5308, t5310) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1274(t1831, t3866, t1307, t1799, t3870, t820);
        let t5314 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1275(t1367, t5187, t820);
        let t5317 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1276(t1341, t1363, t1831, t3781, t3783, t3800, t3803, t3864, t3867, t5259, t5289, t5293, t5303, t5306, t5310, t5314);
        let t5318 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1277(t5257, t5317);
    (t5287, t5289, t5293, t5301, t5303, t5308, t5310, t5314, t5318)
}
