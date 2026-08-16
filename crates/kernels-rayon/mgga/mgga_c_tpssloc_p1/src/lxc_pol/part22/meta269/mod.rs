//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta269 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1412;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1413;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta269(t12129: f64, t17: f64, t521: f64, t9861: f64, t3826: f64, t592: f64, t1285: f64, t2225: f64, t2371: f64, t3691: f64, t1294: f64, t9494: f64, t2535: f64, t1995: f64, t68: f64, t1372: f64, t3787: f64, t215: f64, t535: f64, t9569: f64, t1314: f64, t2559: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12130, t12132, t12133, t12134, t12136, t12138, t12141) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1412(t12129, t17, t521, t9861, t3826, t592, t1285, t2225, t2371, t3691, t1294, t9494);
        let (t12142, t12155, t12171, t12188, t12189) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1413(t2535, t3691, t1995, t68, t1372, t3787, t215, t535, t9569, t1314, t2559);
    (t12130, t12132, t12133, t12134, t12136, t12138, t12141, t12142, t12155, t12171, t12188, t12189)
}
