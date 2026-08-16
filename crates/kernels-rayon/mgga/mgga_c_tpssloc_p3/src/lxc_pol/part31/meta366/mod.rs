//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta366 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1293;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1294;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1295;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1296;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1297;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta366(t5499: f64, t9929: f64, t172: f64, t5522: f64, t763: f64, t184: f64, t5398: f64, t607: f64, t4194: f64, t9864: f64, t9866: f64, t2752: f64, t5664: f64, t40: f64, t12943: f64, t4101: f64, t4205: f64, t4202: f64, t16558: f64, t185: f64, t707: f64, t5392: f64, t634: f64, t75: f64, t3966: f64, t4104: f64, t767: f64, zeta_threshold: f64, t52: f64, t638: f64, t78: f64, t4111: f64, t771: f64, t12922: f64, t12926: f64, t12934: f64, t193: f64, t2522: f64, t4255: f64, t4310: f64, t4314: f64, t766: f64, t776: f64, t9715: f64, t9724: f64, t9726: f64, t9780: f64, t9863: f64, t5575: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16612, t16618, t16622, t16623, t16624, t16625) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1293(t5499, t9929, t172, t5522, t763, t184, t5398, t607, t4194, t9864, t9866, t2752, t5664);
        let (t16629, t16631, t16633, t16636, t16648) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1294(t40, t12943, t4101, t4205, t4202, t16558, t185, t707, t5392, t634, t5398, t75, t3966, t4104, t607, t767, zeta_threshold);
        let t16662 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1295(t52, t5392, t638, t5398, t78, t16558, t3966, t4111, t607, t771, t16648, zeta_threshold);
        let t16666 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1296(t12922, t12926, t12934, t16612, t16618, t16622, t16623, t16624, t16625, t16629, t16631, t16633, t16636, t16662, t193, t2522, t4255, t4310, t4314, t766, t776, t9715, t9724, t9726, t9780, t9863);
        let t16673 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1297(t5575, t68);
    (t16612, t16618, t16622, t16623, t16624, t16629, t16631, t16633, t16636, t16662, t16666, t16673)
}
