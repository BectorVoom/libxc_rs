//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta504 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1863;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1864;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1865;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1866;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta504(t1512: f64, t23041: f64, t4166: f64, t6613: f64, t831: f64, t23053: f64, t4236: f64, t6614: f64, t1878: f64, t23033: f64, t221: f64, t4255: f64, t23125: f64, t23134: f64, t23141: f64, t23144: f64, t25140: f64, t25142: f64, t23042: f64, t23063: f64, t23070: f64, t23084: f64, t25065: f64, t25069: f64, t25071: f64, t25073: f64, t25077: f64, t25080: f64, t25103: f64, t25107: f64, t25109: f64, t25113: f64, t25117: f64, t25121: f64, t25124: f64, t25126: f64, t25128: f64, t25133: f64, t25136: f64, t218: f64, t253: f64, t254: f64, t10109: f64, t1911: f64, t4272: f64, t25036: f64, t25042: f64, t25047: f64, t25049: f64, t25051: f64, t25056: f64, t25061: f64, t259: f64, t2597: f64, t4147: f64, t4301: f64, t6627: f64, t6632: f64, t6663: f64, t7538: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25144, t25146, t25147, t25149, t25151, t25154, t25155) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1863(t1512, t23041, t4166, t6613, t831, t23053, t4236, t6614, t1878, t23033, t221, t4255);
        let t25158 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1864(t25154, t25155, t23125, t23134, t23141, t23144, t25140, t25142, t25144, t25147, t25149, t25151);
        let t25160 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1865(t23042, t23063, t23070, t23084, t25065, t25069, t25071, t25073, t25077, t25080, t25103, t25107, t25109, t25113, t25117, t25121, t25124, t25126, t25128, t25133, t25136, t25158);
        let (t25161, t25168, t25169, t25170, t25173) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1866(t218, t25160, t253, t254, t10109, t1911, t4272, t25036, t25042, t25047, t25049, t25051, t25056, t25061, t259, t2597, t4147, t4301, t6627, t6632, t6663, t7538);
    (t25146, t25154, t25155, t25160, t25161, t25168, t25169, t25170, t25173)
}
