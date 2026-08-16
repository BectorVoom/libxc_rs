//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta192 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk918;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk919;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk920;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta192<F: Float>(t1215: F, t3612: F, t1755: F, t1235: F, t1734: F, t1246: F, t491: F, t5011: F, t1932: F, t475: F, t1751: F, t493: F, t5052: F, t1201: F, t1244: F, t1247: F, t1249: F, t1729: F, t1756: F, t1758: F, t3604: F, t3610: F, t3624: F, t470: F, t494: F, t4964: F, t5064: F, t1241: F, t1238: F, t1252: F, t1761: F, t3487: F, t3593: F, t4941: F, t4943: F, t4945: F, t4947: F, t498: F, t5053: F, t5055: F, t5060: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5068, t5069, t5073, t5076, t5079, t5080, t5084, t5086) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk918::<F>(t1215, t3612, t1755, t1235, t1734, t1246, t491, t5011, t1932, t475, t1751, t493, t5052);
        let t5088 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk919::<F>(t1201, t1244, t1247, t1249, t1729, t1756, t1758, t3604, t3610, t3624, t470, t494, t4964, t5064, t5069, t5073, t5076, t5080, t5084, t5086);
        let (t5089, t5091) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk920::<F>(t1241, t5088, t1238, t1252, t1761, t3487, t3593, t4941, t4943, t4945, t4947, t498, t5053, t5055, t5060);
    (t5068, t5069, t5073, t5076, t5079, t5080, t5084, t5086, t5088, t5089, t5091)
}
