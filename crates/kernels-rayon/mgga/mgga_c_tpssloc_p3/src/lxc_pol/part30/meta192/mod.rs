//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta192 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk918;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk919;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk920;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta192(t1215: f64, t3612: f64, t1755: f64, t1235: f64, t1734: f64, t1246: f64, t491: f64, t5011: f64, t1932: f64, t475: f64, t1751: f64, t493: f64, t5052: f64, t1201: f64, t1244: f64, t1247: f64, t1249: f64, t1729: f64, t1756: f64, t1758: f64, t3604: f64, t3610: f64, t3624: f64, t470: f64, t494: f64, t4964: f64, t5064: f64, t1241: f64, t1238: f64, t1252: f64, t1761: f64, t3487: f64, t3593: f64, t4941: f64, t4943: f64, t4945: f64, t4947: f64, t498: f64, t5053: f64, t5055: f64, t5060: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5068, t5069, t5073, t5076, t5079, t5080, t5084, t5086) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk918(t1215, t3612, t1755, t1235, t1734, t1246, t491, t5011, t1932, t475, t1751, t493, t5052);
        let t5088 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk919(t1201, t1244, t1247, t1249, t1729, t1756, t1758, t3604, t3610, t3624, t470, t494, t4964, t5064, t5069, t5073, t5076, t5080, t5084, t5086);
        let (t5089, t5091) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk920(t1241, t5088, t1238, t1252, t1761, t3487, t3593, t4941, t4943, t4945, t4947, t498, t5053, t5055, t5060);
    (t5068, t5069, t5073, t5076, t5079, t5080, t5084, t5086, t5088, t5089, t5091)
}
