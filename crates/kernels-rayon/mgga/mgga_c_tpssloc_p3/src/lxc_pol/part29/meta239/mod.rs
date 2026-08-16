//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta239 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1121;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1122;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1123;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1124;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1125;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta239(t1458: f64, t671: f64, t1401: f64, t3938: f64, t3941: f64, t4072: f64, t5363: f64, t5371: f64, t577: f64, t2235: f64, t33: f64, t645: f64, t79: f64, t72: f64, t605: f64, t608: f64, t625: f64, t641: f64, t71: f64, t1874: f64, t2314: f64, t4034: f64, t1266: f64, t1873: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5376, t5381, t6486) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1121(t1458, t671, t1401, t3938, t3941, t4072, t5363, t5371, t577, t2235, t33);
        let t6492 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1122(t645, t79, t72);
        let t6495 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1123(t605, t608);
        let (t6503, t6509) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1124(t625, t641, t71);
        let (t6522, t6524, t6525) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1125(t1874, t2314, t4034, t1266, t1873);
    (t5376, t5381, t6486, t6492, t6495, t6503, t6509, t6522, t6524, t6525)
}
