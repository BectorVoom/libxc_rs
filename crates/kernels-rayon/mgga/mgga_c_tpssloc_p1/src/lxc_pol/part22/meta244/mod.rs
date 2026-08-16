//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta244 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1340;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1341;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1342;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1343;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1344;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1345;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1346;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1347;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta244(t10402: f64, t3186: f64, t3062: f64, t820: f64, t3200: f64, t3051: f64, t1005: f64, t3082: f64, t121: f64, t3061: f64, t1008: f64, t349: f64, t1011: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t10403 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1340(t10402, t3186);
        let t10408 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1341(t3062, t820);
        let t10413 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1342(t10402, t3200);
        let t10422 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1343(t3051, t820);
        let (t10436, t10457, t10468, t10469) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1344(t1005, t3082, t121, t3061, t1008);
        let t10470 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1345(t10469, t349);
        let t10471 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1346(t1011);
        let t10472 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1347(t10470, t10471);
    (t10403, t10408, t10413, t10422, t10436, t10457, t10468, t10469, t10470, t10471, t10472)
}
