//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta243 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1102;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1103;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta243(t3: f64, t6470: f64, t1401: f64, t1458: f64, t3941: f64, t5371: f64, t5456: f64, t5493: f64, t577: f64, t2235: f64, t33: f64, t645: f64, t79: f64, t72: f64, t605: f64, t608: f64, t625: f64, t641: f64, t71: f64, t1874: f64, t2314: f64, t4034: f64, t1266: f64, t1873: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6471, t6483, t6486) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1102(t3, t6470, t1401, t1458, t3941, t5371, t5456, t5493, t577, t2235, t33);
        let (t6492, t6495, t6503, t6509, t6522, t6524, t6525) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1103(t645, t79, t72, t605, t608, t625, t641, t71, t1874, t2314, t4034, t1266, t1873);
    (t6471, t6483, t6486, t6492, t6495, t6503, t6509, t6522, t6524, t6525)
}
